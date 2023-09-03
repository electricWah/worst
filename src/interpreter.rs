
//! Simpler interpreter

use std::rc::Rc;
use im_rc::HashMap;
use crate::base::*;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// A set of definition bindings. There are two per stack frame
/// (ambient definitions and local definitions).
#[derive(Default, Clone)]
pub struct DefSet(HashMap<String, Val>);
value!(DefSet);

impl DefSet {
    /// Add a definition.
    pub fn insert(&mut self, key: String, val: Val) {
        self.0.insert(key, val);
    }
    /// Look up a definition.
    pub fn get(&self, key: &str) -> Option<&Val> {
        self.0.get(key)
    }
    /// Absorb the contents of the given [DefSet], overwriting duplicates.
    pub fn merge(&mut self, thee: DefSet) {
        if self.len() == 0 {
            *self = thee;
        } else {
            for (k, v) in thee.0.into_iter() {
                self.0.insert(k, v);
            }
        }
    }
    /// Chainable version of [merge].
    pub fn merged_with(mut self, thee: DefSet) -> DefSet {
        self.merge(thee);
        self
    }

    /// Get an iterator over the keys and values.
    pub fn iter(&self) -> impl Iterator<Item=(&str, &Val)> {
        self.0.iter().map(|(k, v)| (k.as_ref(), v))
    }

    /// Get the number of definitions.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Default)]
struct Frame {
    childs: Vec<ChildFrame>,
    body: List,
    meta: Meta,
    ambient: DefSet,
    locals: DefSet,
}

impl Frame {
    fn is_empty(&self) -> bool {
        self.childs.is_empty() && self.body.is_empty()
    }

    fn from_list_defs(body: List, meta: Meta, ambient: DefSet) -> Self {
        Frame { body, meta, ambient, ..Default::default() }
    }

    fn insert_local(&mut self, key: String, val: Val) {
        self.locals.0.insert(key, val);
    }
    fn lookup(&self, key: &str) -> Option<&Val> {
        self.locals.0.get(key).or_else(|| self.ambient.0.get(key))
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
value!(Builtin);
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
            let meta = v.meta_ref().clone();
            let l = v.try_downcast::<List>().ok().unwrap();
            let defs = self.get_meta_type::<DefSet>(l.meta_ref()).cloned().unwrap_or_default();
            self.frame.childs.push(ChildFrame::Frame(Frame::from_list_defs(l.into_inner(), meta, defs)));
        } else {
            self.stack_push(v);
        }
        Ok(())
    }

    /// Same as [eval_next], but attaching an ambient defset beforehand.
    pub fn eval_list_next(&mut self, v: ValOf<List>) {
        let defs = {
            if let Some(real) = self.get_meta_type::<DefSet>(v.meta_ref()).cloned() {
                real
            } else {
                self.ambients_ref().clone().merged_with(self.locals_ref().clone())
            }
        };
        let meta = v.meta_ref().clone();
        self.frame.childs.push(ChildFrame::Frame(Frame::from_list_defs(v.into_inner(), meta, defs)));
    }

    /// Evaluate this FnOnce in the next [run] step. See [eval_next].
    pub fn eval_next_once<T: 'static + FnOnce(&mut Interpreter) -> BuiltinRet>
        (&mut self, f: T) {
        self.frame.childs.push(ChildFrame::Once(Box::new(f)));
    }

    /// Same as [eval_next], or [eval_list_next] if `v` is a [List].
    pub fn eval_any_next(&mut self, v: Val) -> BuiltinRet {
        match v.try_downcast::<List>() {
            Ok(l) => self.eval_list_next(l),
            Err(v) => self.eval_next(v)?,
        }
        Ok(())
    }

    /// Find a definition in the current local and then closure environments.
    pub fn resolve_definition(&self, name: &str) -> Option<&Val> {
        self.frame.lookup(name)
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
        let def = self.add_meta_type(def.into(), self.ambients_ref().clone());
        self.frame.insert_local(name, def);
    }

    /// Add the given value to local definitions.
    pub fn add_definition(&mut self, name: impl Into<String>, def: impl Into<Val>) {
        self.frame.insert_local(name.into(), def.into());
    }

    /// Add the given builtin to the ambient definition set.
    pub fn add_builtin(&mut self, name: impl Into<String>, def: impl Into<Builtin>) {
        self.frame.insert_local(name.into(), Val::from(def.into()));
    }

    /// Get a reference to the current ambient [DefSet].
    pub fn ambients_ref(&self) -> &DefSet { &self.frame.ambient }
    /// Get a reference to the current locals [DefSet].
    pub fn locals_ref(&self) -> &DefSet { &self.frame.locals }
    /// Get a mutable reference to the current ambient [DefSet].
    pub fn ambients_mut(&mut self) -> &mut DefSet { &mut self.frame.ambient }
    /// Get a mutable reference to the current locals [DefSet].
    pub fn locals_mut(&mut self) -> &mut DefSet { &mut self.frame.locals }

    /// Get a reference to the current frame [Meta] (from the list being evaluated).
    pub fn frame_meta_ref(&self) -> &Meta { &self.frame.meta }

    /// Get a list of all Meta entries for stack frames
    /// (starting from the current one and working up to the topmost frame).
    pub fn stack_meta_refs(&self) -> impl Iterator<Item = &Meta> {
        vec![&self.frame.meta].into_iter()
            .chain(self.parents.iter().map(|p| &p.meta))
    }

    /// Mark the given value as an error and return it.
    pub fn set_error(&mut self, v: impl Into<Val>) -> Val {
        self.add_meta_type(v.into(), IsError)
    }

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
        let val = self.set_error(v);
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
            if !frame.is_empty() {
                self.frame.childs.push(ChildFrame::Frame(frame));
            }
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
    fn get_meta_type<'a, T: Value>(&self, meta: &'a Meta) -> Option<&'a T> {
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


