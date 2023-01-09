
//! Simpler interpreter

use std::rc::Rc;
use crate::base::*;
use crate::interpreter::{DefSet, DefScope};

#[derive(Default)]
struct Frame {
    childs: Vec<ChildFrame>,
    body: List,
    defenv: DefSet,
    locals: DefSet,
}

impl Frame {
    fn is_empty(&self) -> bool {
        self.childs.is_empty() && self.body.is_empty()
    }
    fn get_definition(&self, name: impl AsRef<str>, scope: Option<DefScope>) -> Option<&Val> {
        if scope != Some(DefScope::DefEnv) {
            if let def@Some(_) = self.locals.get(name.as_ref()) {
                return def;
            }
        }
        if scope != Some(DefScope::Local) {
            if let def@Some(_) = self.defenv.get(name.as_ref()) {
                return def;
            }
        }
        None
    }
    fn all_defs(&self) -> DefSet {
        let mut defs = self.defenv.clone();
        defs.append(&self.locals);
        defs
    }
    fn from_list(l: ValOf<List>) -> Self {
        // TODO name
        let defenv = l.meta_ref().first_ref::<DefSet>().cloned().unwrap_or_default();
        let body = l.into_inner();
        Frame { defenv, body, ..Frame::default() }
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

    /// Check if there is anything else left to evaluate.
    pub fn is_complete(&self) -> bool {
        self.frame.is_empty() && self.parents.is_empty()
    }

    /// Run until the next pause or error, or to completion.
    pub fn run(&mut self) -> Result<(), Val> {
        loop {
            if let Some(child) = self.frame.childs.pop() {
                match child {
                    ChildFrame::Builtin(b) => b.0(self)?,
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

    /// Evaluate this FnOnce in the next [run] step. See [eval_next].
    pub fn eval_next_once<T: 'static + FnOnce(&mut Interpreter) -> BuiltinRet>
        (&mut self, f: T) {
        self.frame.childs.push(ChildFrame::Once(Box::new(f)));
    }

    fn eval_next_resolve(&mut self, v: &Symbol) -> BuiltinRet {
        if let Some(def) = self.frame.get_definition(v, None).cloned() {
            self.eval_next(def)?;
        } else {
            self.error(List::from(vec!["undefined".to_symbol().into(),
                                       v.clone().into()]))?;
        }
        Ok(())
    }

    /// Add a definition to the current stack frame.
    /// Inserts meta values such as name and a static environment.
    pub fn define(&mut self, name: impl Into<String>, def: impl Into<Val>) {
        let mut def = def.into();
        let m = def.meta_mut();
        m.push(self.all_definitions());
        self.frame.locals.insert(name.into(), def);
    }

    /// Insert the given value into one of the definition scopes.
    pub fn add_definition(&mut self, name: impl Into<String>, def: impl Into<Val>, scope: DefScope) {
        match scope {
            DefScope::Local => self.frame.locals.insert(name, def),
            DefScope::DefEnv => self.frame.defenv.insert(name, def),
        }
    }

    /// Add the given builtin to the global env.
    pub fn add_builtin(&mut self, name: impl Into<String>, def: impl Into<Builtin>) {
        self.add_definition(name, def.into(), DefScope::DefEnv);
    }

    /// Get all available definitions.
    pub fn all_definitions(&self) -> DefSet {
        self.frame.all_defs()
    }

    // maybe all of these should be within List
    // and just have stack_ref and stack_mut
    /// Get a reference to the stack
    pub fn stack_ref(&self) -> &List { &self.stack }
    /// Get a mutable reference to the stack
    pub fn stack_mut(&mut self) -> &mut List { &mut self.stack }
    /// Put something on top of the stack
    pub fn stack_push(&mut self, v: impl Into<Val>) { self.stack.push(v.into()); }
    /// Pop the top thing off the stack
    pub fn stack_pop_val(&mut self) -> BuiltinRet<Val> {
        self.stack.pop().ok_or("stack-empty".to_symbol().into())
    }
    /// Get the top thing off the stack without popping it
    pub fn stack_top_val(&mut self) -> BuiltinRet<Val> {
        self.stack.top().cloned().ok_or("stack-empty".to_symbol().into())
    }

    /// Pop the top thing off the stack if it has the given type
    pub fn stack_pop<T: Value>(&mut self) -> BuiltinRet<ValOf<T>> {
        let v = self.stack_pop_val()?;
        v.try_downcast::<T>().map_err(|v| List::from(vec![
            "wrong-type".to_symbol().into(),
            v, std::any::type_name::<T>().to_string().into(),
        ]).into())
    }

    /// Get a mutable reference to the remaining code in the current stack frame.
    pub fn body_mut(&mut self) -> &mut List { &mut self.frame.body }

    /// Get the next item in the current stack body.
    pub fn body_next(&mut self) -> BuiltinRet<Val> {
        self.body_mut().pop().ok_or("quote-nothing".to_symbol().into())
    }

    /// Pause evaluation. [run] will return with this value.
    pub fn pause(&self, v: impl Into<Val>) -> BuiltinRet {
        Err(v.into())
    }
    /// Pause evaluation with an error. [run] will return with this value.
    pub fn error(&self, v: impl Into<Val>) -> BuiltinRet {
        Err(IsError::add(v.into()))
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
    fn test_uplevel() {
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
    fn test_uplevel_closure() {
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
    fn test_uplevel2() {
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

}


