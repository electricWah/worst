
//! Simpler interpreter

use std::rc::Rc;
use im_rc::{HashMap, HashSet};
use crate::base::*;
use std::any::TypeId;

/// A collection of definitions, as made famous by local variables.
#[derive(Default, Clone)]
pub struct DefSet {
    inner: HashMap<String, Val>,
}
impl Value for DefSet {}

impl DefSet {
    /// Add a definition.
    pub fn insert(&mut self, key: String, val: Val) {
        self.inner.insert(key, val);
    }
    /// Remove a definition.
    pub fn remove(&mut self, key: &str) -> Option<Val> {
        self.inner.remove(key)
    }

    /// Look for a definition by name.
    pub fn get(&self, key: &str) -> Option<&Val> {
        self.inner.get(key)
    }

    /// Check whether the definition exists.
    pub fn contains(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    /// Take all definitions from [thee] and put them in [self],
    /// overwriting duplicate keys in [self] with the values from [thee].
    pub fn extend(&mut self, thee: &DefSet) {
        let new = thee.inner.iter().map(|(k, v)| (k.clone(), v.clone()));
        self.inner.extend(new);
    }

    /// How many entries there are.
    pub fn len(&self) -> usize { self.inner.len() }

    /// Whether there are no entries.
    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
}

#[derive(Default, Clone)]
struct DefEnvEntry {
    local: Option<Val>,
    ambient: Option<Val>,
}

/// An environment of definitions for a stack frame.
#[derive(Default, Clone)]
pub struct DefEnv {
    names: HashSet<String>,
    entries: HashMap<String, DefEnvEntry>,
}
impl Value for DefEnv {}

impl DefEnv {

//     /// Get a reference to the current local variables.
//     pub fn locals_ref(&self) -> &DefSet { &self.locals }
//     /// Get a mutable reference to the current local variables.
//     pub fn locals_mut(&mut self) -> &mut DefSet { &mut self.locals }

    /// Look up a definition.
    fn lookup(&self, key: &str) -> Option<&Val> {
        self.entries.get(key).and_then(|s| s.local.as_ref().or(s.ambient.as_ref()))
    }

    /// Look up a definition in locals only.
    pub fn get_local(&self, key: &str) -> Option<&Val> {
        self.entries.get(key).and_then(|s| s.local.as_ref())
    }

    /// Start a new set of locals.
    /// The current locals set becomes an ambient definition.
    pub fn new_locals(&mut self) {
        self.names.clear();
    }

    /// Insert a new local value.
    pub fn insert(&mut self, key: String, val: Val) {
        if self.names.insert(key.clone()).is_some() {
            // new local: previous local (if any) should be ambient
            let entry = self.entries.entry(key).or_insert_with(DefEnvEntry::default);
            entry.ambient = entry.local.take();
            entry.local = Some(val);
        } else {
            // overwriting existing local
            let entry = self.entries.entry(key).or_insert_with(DefEnvEntry::default);
            entry.local = Some(val);
        }
    }

    /// TODO remove this and DefSet entirely
    pub fn extend_locals(&mut self, locals: DefSet) {
        for (k, v) in locals.inner.iter() {
            self.insert(k.clone(), v.clone());
        }
    }

    /// Get a DefSet of the current locals
    pub fn locals(&self) -> DefSet {
        let mut defs = DefSet::default();
        for k in self.names.iter() {
            defs.insert(k.clone(), self.get_local(k).unwrap().clone());
        }
        defs
    }
}

/// Metadata for a list definition stating its name.
#[derive(Clone)]
pub struct DefineName(String);
impl Value for DefineName {}
impl DefineName {
    /// Create a thing
    pub fn new(s: impl Into<String>) -> Self { DefineName(s.into()) }
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
    fn from_list(l: ValOf<List>) -> Self {
        let ds = l.meta_ref().get_ref::<DefEnv>().cloned().unwrap_or_default();
        Self::from_list_env(l.into_inner(), ds)
    }
}

enum ChildFrame {
    Frame(Frame),
    Builtin(Builtin),
    Once(Box<dyn FnOnce(&mut Interpreter) -> BuiltinRet>),
}

/// A Worst interpreter.
#[derive(Default)]
pub struct Interpreter {
    frame: Frame,
    parents: Vec<Frame>,
    stack: List,
}
impl Value for Interpreter {}

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
            self.frame.childs.push(ChildFrame::Frame(Frame::from_list(l)));
        } else {
            self.stack_push(v);
        }
        Ok(())
    }

    /// Same as [eval_next], but attaching a defenv beforehand.
    // TODO always use list's defenv then current defenv
    pub fn eval_list_next(&mut self, v: ValOf<List>) {
        let defs = {
            if let Some(real) = v.meta_ref().get_ref::<DefEnv>().cloned() {
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
        let mut def = def.into();
        let m = def.meta_mut();
        m.insert(DefineName(name.clone()));
        m.insert(self.defenv_ref().clone());
        self.frame.defs.insert(name, def);
    }

    /// Add the given value to local definitions.
    pub fn add_definition(&mut self, name: impl Into<String>, def: impl Into<Val>) {
        self.frame.defs.insert(name.into(), def.into());
    }

    /// Add the given builtin to the ambient definition set.
    pub fn add_builtin(&mut self, name: impl Into<String>, def: impl Into<Builtin>) {
        self.frame.defs.insert(name.into(), Val::from(def.into()));
    }

    // /// Get all local definitions.
    // /// These are usually the definitions added by code in the current stack frame
    // /// and will be lost once completely executed.
    // pub fn locals_ref(&self) -> &DefSet { self.frame.defs.locals_ref() }
    // /// Get a mutable reference to the local definitions for the current stack frame.
    // pub fn locals_mut(&mut self) -> &mut DefSet { self.frame.defs.locals_mut() }

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
    /// Put something on top of the stack, or false
    pub fn stack_push_option<T: Into<Val>>(&mut self, v: Option<T>) {
        if let Some(v) = v {
            self.stack.push(v.into());
        } else {
            self.stack.push(false);
        }
    }
    /// Put an Ok value, or Err with IsError set, on top of the stack.
    pub fn stack_push_result<T: Into<Val>, E: Into<Val>>(&mut self, v: Result<T, E>) {
        match v {
            Ok(ok) => self.stack_push(ok),
            Err(e) => self.stack_push(IsError::add(e)),
        }
    }
    /// Pop the top thing off the stack
    pub fn stack_pop_val(&mut self) -> BuiltinRet<Val> {
        Self::or_err(self.stack.pop(), "stack-empty")
    }
    /// Get the top thing off the stack without popping it
    pub fn stack_top_val(&mut self) -> BuiltinRet<Val> {
        Self::or_err(self.stack.top().cloned(), "stack-empty")
    }

    /// Pop the top thing off the stack if it has the given type
    pub fn stack_pop<T: Value>(&mut self) -> BuiltinRet<ValOf<T>> {
        let v = self.stack_pop_val()?;
        v.try_downcast::<T>().map_err(|v| {
            let vty = v.val_type_id();
            IsError::add(List::from(vec![
                "wrong-type".to_symbol().into(),
                v, vty.into(), TypeId::of::<T>().into(),
            ]))
        })
    }

    /// Get the top thing off the stack without popping it,
    /// if it has the given type
    pub fn stack_top<T: Value>(&mut self) -> BuiltinRet<ValOf<T>> {
        let v = self.stack_pop::<T>()?;
        self.stack_push(v.clone());
        Ok(v)
    }
    /// Get a mutable reference to the remaining code in the current stack frame.
    pub fn body_mut(&mut self) -> &mut List { &mut self.frame.body }

    /// Get the next item in the current stack frame body.
    pub fn body_next_val(&mut self) -> BuiltinRet<Val> {
        Self::or_err(self.body_mut().pop(), "quote-nothing")
    }
    /// Get the next `T` in the current stack frame body.
    pub fn body_next<T: Value>(&mut self) -> BuiltinRet<ValOf<T>> {
        self.body_next_val()?.try_downcast::<T>().map_err(|v| {
            let vty = v.val_type_id();
            IsError::add(List::from(vec![
                "wrong-type".to_symbol().into(),
                v, vty.into(), TypeId::of::<T>().into(),
            ]))
        })
    }

    /// Pause evaluation. [run] will return with this value.
    pub fn pause(&self, v: impl Into<Val>) -> BuiltinRet {
        Err(v.into())
    }
    /// Pause evaluation with an error. [run] will return with this value.
    pub fn error(&self, v: impl Into<Val>) -> BuiltinRet {
        Err(IsError::add(v.into()))
    }

    fn or_err<T>(v: Option<T>, err: impl Into<Symbol>) -> BuiltinRet<T> {
        match v {
            Some(v) => Ok(v),
            None => Err(IsError::add(err.into())),
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
        assert!(IsError::is_error(&err));
        assert_eq!(err.downcast_ref::<Symbol>(),
                   Some(&"stack-empty".to_symbol()));
    }

}


