
//! Simpler interpreter

use std::rc::Rc;
use im_rc::{HashMap, HashSet};
use crate::base::*;
use std::any::TypeId;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Default, Clone)]
struct DefEnvEntry {
    local: Option<Val>,
    ambient: Option<Val>,
}

/// An environment of definitions for a stack frame.
#[derive(Default, Clone)]
pub struct DefEnv {
    locals: HashSet<String>,
    entries: HashMap<String, DefEnvEntry>,
}
impl Value for DefEnv {}

impl DefEnv {
    /// Look up a definition.
    pub fn lookup(&self, key: &str) -> Option<&Val> {
        self.entries.get(key).and_then(|s| s.local.as_ref().or(s.ambient.as_ref()))
    }

    /// Look up a definition in locals only.
    pub fn get_local(&self, key: &str) -> Option<&Val> {
        if self.locals.contains(key) {
            self.entries.get(key).and_then(|s| s.local.as_ref())
        } else { None }
    }

    /// Start a new set of locals.
    /// The current locals set becomes an ambient definition.
    pub fn new_locals(&mut self) {
        self.locals.clear();
    }

    /// Insert a new local value.
    pub fn insert_local(&mut self, key: String, val: Val) {
        if self.locals.insert(key.clone()).is_none() {
            // new local, perhaps
            let entry = self.entries.entry(key).or_insert_with(DefEnvEntry::default);
            // after new_locals, entry.local is now actually ambient
            entry.ambient = entry.local.take();
            entry.local = Some(val);
        } else {
            // overwriting existing local
            let entry = self.entries.entry(key).or_insert_with(DefEnvEntry::default);
            entry.local = Some(val);
        }
    }

    /// Get an iterator over the local definitions
    pub fn locals_iter(&self) -> impl Iterator<Item=(&str, &Val)> {
        self.locals.iter().filter_map(|k| {
            self.entries.get(k)
                .and_then(|e| e.local.as_ref())
                .map(|l| (k.as_ref(), l))
        })
    }

    /// Get an iterator over all definitions
    /// (returning also whether each definition was local or not)
    pub fn iter(&self) -> impl Iterator<Item=(&str, &Val, bool)> {
        self.entries.iter().filter_map(|(k, e)| {
            if e.local.is_some() {
                e.local.as_ref().map(|v| (k.as_ref(), v, true))
            } else {
                e.ambient.as_ref().map(|v| (k.as_ref(), v, false))
            }
        })
    }

    /// Copy local definitions into self's local definitions.
    pub fn extend_locals(&mut self, locals: DefEnv) {
        for (l, def) in locals.locals_iter() {
            let l = String::from(l);
            self.locals.insert(l.clone());
            let entry = self.entries.entry(l).or_insert_with(DefEnvEntry::default);
            entry.local = Some(def.clone());
        }
    }
}

#[derive(Default)]
struct Frame {
    childs: Vec<ChildFrame>,
    body: List,
    #[allow(dead_code)]
    name: Option<String>,
    defs: DefEnv,
}

impl Frame {
    fn is_empty(&self) -> bool {
        self.childs.is_empty() && self.body.is_empty()
    }
    // TODO clean up
    fn from_list_env(body: List, defs: DefEnv) -> Self {
        let childs = vec![];
        // TODO name
        let name = None; // l.meta_ref().get_ref::<DefineName>().cloned().map(|d| d.0);
        Frame { childs, body, name, defs, }
    }
}

enum ChildFrame {
    Frame(Frame),
    Builtin(Builtin),
    Once(Box<dyn FnOnce(&mut Interpreter) -> BuiltinRet>),
}

/// A Worst interpreter.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Default)]
pub struct Interpreter {
    frame: Frame,
    parents: Vec<Frame>,
    stack: List,
    uniques: UniqueGen,
}

/// Return type for [Builtin] functions.
pub type BuiltinRet<R = ()> = Result<R, Val>;

/// A definition written in Rust rather than Worst.
#[derive(Clone)]
pub struct Builtin(Rc<dyn Fn(&mut Interpreter) -> BuiltinRet>);
impl Value for Builtin {}
impl<T: 'static + Fn(&mut Interpreter) -> BuiltinRet> From<T> for Builtin {
    fn from(f: T) -> Self {
        Builtin(Rc::new(f))
    }
}

impl Interpreter {

    /// Create a new "inner" interpreter that is otherwise empty.
    /// This must be used in order to correctly share values between interpreters.
    pub fn new_inner_empty(&self) -> Interpreter {
        Interpreter { uniques: self.uniques.clone(), ..Default::default() }
    }

    /// Create a new interpreter that will evaluate the given code.
    pub fn new(body: impl Into<List>) -> Self {
        let mut i = Interpreter::default();
        i.frame.body = body.into();
        i
    }

    /// Make the interpreter stop doing things,
    /// but leave its toplevel definitions intact.
    pub fn reset(&mut self) {
        while self.enter_parent_frame().is_ok() {}
        self.frame.childs = vec![];
        self.frame.body = List::default();
    }

    /// Get a basic call stack.
    /// The returned list starts with the current stack frame and ends at the root.
    pub fn call_stack_names(&self) -> Vec<Option<String>> {
        let mut acc = vec![self.frame.name.clone()];
        for f in self.parents.iter() {
            acc.push(f.name.clone());
        }
        acc
    }

    /// Check if there is anything else left to evaluate.
    pub fn is_complete(&self) -> bool {
        self.frame.is_empty() && self.parents.is_empty()
    }

    /// Run until the next pause or error, or to completion.
    pub fn run(&mut self) -> Result<(), Val> {
        loop {
            if let Some(child) = self.frame.childs.pop() {
                match child {
                    ChildFrame::Builtin(b) => {
                        b.0(self)?
                    },
                    ChildFrame::Once(f) => f(self)?,
                    ChildFrame::Frame(mut f) => {
                        std::mem::swap(&mut self.frame, &mut f);
                        self.parents.push(f);
                    },
                }
            } else if let Some(next) = self.frame.body.pop() {
                // everything except symbols is literal
                if let Some(s) = next.downcast_ref::<Symbol>() {
                    self.eval_next_resolve(s)?;
                } else {
                    self.stack_push(next);
                }
            } else if let Some(mut frame) = self.parents.pop() {
                std::mem::swap(&mut self.frame, &mut frame);
            } else {
                return Ok(());
            }
        }
    }

    /// Evaluate this thing in the next [run] step.
    /// Multiple eval_next-ed things will be run in reverse order.
    /// If it's is a list, it should already have a ambients attached.
    pub fn eval_next(&mut self, v: impl Into<Val>) -> BuiltinRet {
        let v = v.into();
        if let Some(s) = v.downcast_ref::<Symbol>() {
            self.eval_next_resolve(s)?;
        } else if let Some(b) = v.downcast_ref::<Builtin>() {
            self.frame.childs.push(ChildFrame::Builtin(b.clone()));
        } else if v.is::<List>() {
            let l = v.try_downcast::<List>().ok().unwrap();
            let defenv = self.get_meta_type::<DefEnv>(l.meta_ref()).cloned().unwrap_or_default();
            self.frame.childs.push(ChildFrame::Frame(Frame::from_list_env(l.into_inner(), defenv)));
        } else {
            self.stack_push(v);
        }
        Ok(())
    }

    /// Same as [eval_next], but attaching a defenv beforehand.
    // TODO always use list's defenv then current defenv
    pub fn eval_list_next(&mut self, v: ValOf<List>) {
        let defs = {
            if let Some(real) = self.get_meta_type::<DefEnv>(v.meta_ref()).cloned() {
                real
            } else {
                let mut defs = self.defenv_ref().clone();
                defs.new_locals();
                defs
            }
        };
        self.frame.childs.push(ChildFrame::Frame(Frame::from_list_env(v.into_inner(), defs)));
    }

    /// Evaluate this FnOnce in the next [run] step. See [eval_next].
    pub fn eval_next_once<T: 'static + FnOnce(&mut Interpreter) -> BuiltinRet>
        (&mut self, f: T) {
        self.frame.childs.push(ChildFrame::Once(Box::new(f)));
    }

    /// Find a definition in the current local and then closure environments.
    pub fn resolve_definition(&self, name: &str) -> Option<&Val> {
        self.frame.defs.lookup(name)
    }

    fn eval_next_resolve(&mut self, v: &Symbol) -> BuiltinRet {
        if let Some(def) = self.resolve_definition(v.as_ref()) {
            self.eval_next(def.clone())?;
        } else {
            self.error(List::from(vec!["undefined".to_symbol().into(),
                                       v.clone().into()]))?;
        }
        Ok(())
    }

    /// Add a definition to the current stack frame.
    /// Inserts meta values such as name and a static environment.
    pub fn define(&mut self, name: impl Into<String>, def: impl Into<Val>) {
        let name = name.into();
        let def = self.add_meta_type(def.into(), self.defenv_ref().clone());
        self.frame.defs.insert_local(name, def);
    }

    /// Add the given value to local definitions.
    pub fn add_definition(&mut self, name: impl Into<String>, def: impl Into<Val>) {
        self.frame.defs.insert_local(name.into(), def.into());
    }

    /// Add the given builtin to the ambient definition set.
    pub fn add_builtin(&mut self, name: impl Into<String>, def: impl Into<Builtin>) {
        self.frame.defs.insert_local(name.into(), Val::from(def.into()));
    }

    /// Get a reference to the current [DefEnv].
    pub fn defenv_ref(&self) -> &DefEnv { &self.frame.defs }
    /// Get a mutable reference to the current [DefEnv].
    pub fn defenv_mut(&mut self) -> &mut DefEnv { &mut self.frame.defs }

    // maybe all of these should be within List
    // and just have stack_ref and stack_mut
    /// Get a reference to the stack
    pub fn stack_ref(&self) -> &List { &self.stack }
    /// Get a mutable reference to the stack
    pub fn stack_mut(&mut self) -> &mut List { &mut self.stack }
    /// Put something on top of the stack
    pub fn stack_push(&mut self, v: impl Into<Val>) { self.stack.push(v.into()); }
    /// Push something on top of the stack, marking it as an error
    pub fn stack_push_error(&mut self, v: impl Into<Val>) {
        let val = self.add_meta_type(v.into(), IsError);
        self.stack.push(val);
    }
    /// Put something on top of the stack, or false
    pub fn stack_push_option<T: Into<Val>>(&mut self, v: Option<T>) {
        if let Some(v) = v {
            self.stack.push(v.into());
        } else {
            self.stack.push(false);
        }
    }
    /// Put something on top of the stack, or false with IsError set
    pub fn stack_push_opterr<T: Into<Val>>(&mut self, v: Option<T>) {
        if let Some(v) = v {
            self.stack.push(v.into());
        } else {
            self.stack_push_error(false);
        }
    }
    /// Put an Ok value, or Err with IsError set, on top of the stack.
    pub fn stack_push_result<T: Into<Val>, E: Into<Val>>(&mut self, v: Result<T, E>) {
        match v {
            Ok(ok) => self.stack_push(ok),
            Err(e) => self.stack_push_error(e),
        }
    }
    /// Pop the top thing off the stack
    pub fn stack_pop_val(&mut self) -> BuiltinRet<Val> {
        let r = self.stack.pop();
        self.or_err(r, "stack-empty")
    }
    /// Get the top thing off the stack without popping it
    pub fn stack_top_val(&mut self) -> BuiltinRet<Val> {
        self.or_err(self.stack.top().cloned(), "stack-empty")
    }

    /// Pop the top thing off the stack if it has the given type
    pub fn stack_pop<T: Value>(&mut self) -> BuiltinRet<ValOf<T>> {
        let v = self.stack_pop_val()?;
        v.try_downcast::<T>().map_err(|v| {
            let vty = v.val_type_id();
            self.add_meta_type(List::from(vec![
                "wrong-type".to_symbol().into(),
                v, vty.into(), TypeId::of::<T>().into(),
                std::any::type_name::<T>().to_string().into(),
            ]).into(), IsError)
        })
    }

    /// Get the top thing off the stack without popping it,
    /// if it has the given type
    pub fn stack_top<T: Value>(&mut self) -> BuiltinRet<ValOf<T>> {
        let v = self.stack_pop::<T>()?;
        self.stack_push(v.clone());
        Ok(v)
    }
    /// Get a reference to the remaining code in the current stack frame.
    pub fn body_ref(&self) -> &List { &self.frame.body }
    /// Get a mutable reference to the remaining code in the current stack frame.
    pub fn body_mut(&mut self) -> &mut List { &mut self.frame.body }

    /// Pause evaluation. [run] will return with this value.
    pub fn pause(&self, v: impl Into<Val>) -> BuiltinRet {
        Err(v.into())
    }
    /// Pause evaluation with an error. [run] will return with this value.
    pub fn error(&mut self, v: impl Into<Val>) -> BuiltinRet {
        Err(self.add_meta_type(v.into(), IsError))
    }

    fn or_err<T>(&mut self, v: Option<T>, err: impl Into<Symbol>) -> BuiltinRet<T> {
        match v {
            Some(v) => Ok(v),
            None => Err(self.add_meta_type(Val::from(err.into()), IsError)),
        }
    }

    /// Run the rest of this function in the parent stack frame.
    pub fn enter_parent_frame(&mut self) -> BuiltinRet {
        if let Some(mut frame) = self.parents.pop() {
            std::mem::swap(&mut self.frame, &mut frame);
            self.frame.childs.push(ChildFrame::Frame(frame));
            Ok(())
        } else {
            self.error("root-uplevel".to_symbol())
        }
    }

    /// Get a mutable reference to the interpreter's [UniqueGen]
    /// for creating and querying unique values.
    pub fn uniques_mut(&mut self) -> &mut UniqueGen { &mut self.uniques }

    fn add_meta_type<T: 'static + Into<Val>>(&mut self, mut v: Val, m: T) -> Val {
        let mu = self.uniques.get_type::<T>();
        v.meta_mut().insert_val(mu, m.into());
        v
    }
    fn get_meta_type<'a, T: 'static>(&self, meta: &'a Meta) -> Option<&'a T> {
        if let Some(tu) = self.uniques.lookup_type::<T>() {
            meta.get_ref::<T>(&tu)
        } else { None }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    /// Pop the top of the stack and assume it's a T
    fn pop_cast<T: Value + Clone>(i: &mut Interpreter) -> T {
        i.stack_pop::<T>().map_err(|_| "wrong type").unwrap().into_inner()
    }

    #[test]
    fn interp_empty() {
        assert!(Interpreter::default().run().is_ok());
    }

    #[test]
    fn interp_simple_1() {
        // stack
        let mut i = Interpreter::new(vec![7.into()]);
        assert!(i.stack_ref().is_empty());
        assert!(i.run().is_ok());
        assert_eq!(pop_cast::<i64>(&mut i), 7);
        assert!(i.stack_ref().is_empty());
    }

    fn toplevel_def(i: &mut Interpreter) -> BuiltinRet {
        i.stack_push("yay".to_string());
        Ok(())
    }

    #[test]
    fn interp_def_1() {
        let mut i =
            Interpreter::new(vec![
                "thingy".to_symbol().into(),
                "test".to_symbol().into(),
            ]);
        i.add_builtin("test", |i: &mut Interpreter| {
            i.stack_push("hello".to_string());
            Ok(())
        });
        i.add_builtin("thingy", toplevel_def);
        assert!(i.run().is_ok());
        assert_eq!(pop_cast::<String>(&mut i), "hello".to_string());
        assert_eq!(pop_cast::<String>(&mut i), "yay".to_string());
        assert!(i.stack_ref().is_empty());
    }

    #[test]
    fn quoting() {
        let mut i =
            Interpreter::new(vec![
                "quote".to_symbol().into(),
                "egg".to_symbol().into(),
            ]);
        i.add_builtin("quote", |i: &mut Interpreter| {
            let q = i.body_mut().pop().expect("quote");
            i.stack_push(q);
            Ok(())
        });
        assert!(i.run().is_ok());
        assert_eq!(pop_cast::<Symbol>(&mut i), "egg".to_symbol());
        assert!(i.stack_ref().is_empty());
    }

    #[test]
    fn uplevel() {
        let mut i =
            Interpreter::new(vec![
                "thing".to_symbol().into(),
                "egg".to_symbol().into(),
            ]);
        i.add_builtin("upquote", |i: &mut Interpreter| {
            i.enter_parent_frame()?;
            let q = i.body_mut().pop().expect("quote");
            i.stack_push(q);
            Ok(())
        });
        i.define("thing", Val::from(List::from(vec![ "upquote".to_symbol().into() ])));
        assert!(i.run().is_ok());
        assert_eq!(pop_cast::<Symbol>(&mut i), "egg".to_symbol());
        assert!(i.stack_ref().is_empty());
    }

    #[test]
    fn uplevel_closure() {
        let mut i =
            Interpreter::new(vec![
                "thing".to_symbol().into(),
            ]);
        i.add_builtin("upfive", |i: &mut Interpreter| {
            let five = "five".to_symbol();
            i.enter_parent_frame()?;
            i.stack_push(five);
            Ok(())
        });
        i.define("thing", Val::from(List::from(vec![ "upfive".to_symbol().into() ])));
        assert!(i.run().is_ok());
        assert_eq!(pop_cast::<Symbol>(&mut i), "five".to_symbol());
        assert!(i.stack_ref().is_empty());
    }

    #[test]
    fn uplevel2() {
        let mut i =
            Interpreter::new(vec![
                "thing1".to_symbol().into(),
                "egg".to_symbol().into(),
            ]);
        i.add_builtin("upquote2", |i: &mut Interpreter| {
            i.enter_parent_frame()?;
            i.enter_parent_frame()?;
            let q = i.body_mut().pop().expect("quote");
            i.stack_push(q);
            Ok(())
        });
        i.define("thing2", Val::from(List::from(vec![ "upquote2".to_symbol().into() ])));
        i.define("thing1", Val::from(List::from(vec![ "thing2".to_symbol().into() ])));
        assert!(i.run().is_ok());
        assert_eq!(pop_cast::<Symbol>(&mut i), "egg".to_symbol());
        assert!(i.stack_ref().is_empty());
    }

    #[test]
    fn stack_pop_empty() {
        let mut i =
            Interpreter::new(vec![ "drop".to_symbol().into(), ]);
        i.add_builtin("drop", |i: &mut Interpreter| {
            i.stack_pop_val()?;
            Ok(())
        });
        let err = i.run().unwrap_err();
        // assert!(IsError::is_error(&err));
        assert_eq!(err.downcast_ref::<Symbol>(),
                   Some(&"stack-empty".to_symbol()));
    }

}


