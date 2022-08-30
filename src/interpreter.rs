
//! An [Interpreter] for Worst code.

use crate::impl_value;
use crate::base::*;
use crate::list::List;

mod base;
mod handle;
pub use base::{Handle, DefineMeta, Builtin, DefSet, ClosureEnv};
use base::*;
pub use self::handle::*;

/// A Worst interpreter, the thing you define functions for and run code in and stuff.
#[derive(Default)]
pub struct Interpreter {
    frame: ListFrame,
    parents: Vec<ListFrame>,
    stack: List,
    defstacks: DefStacks,
}

impl std::fmt::Debug for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<paused interpreter>")
    }
}

impl_value!(Interpreter, value_debug::<Interpreter>());

impl Interpreter {

    /// Check if there is anything else left to evaluate.
    pub fn is_complete(&self) -> bool {
        self.frame.is_empty() && self.parents.is_empty()
    }

    /// Add a definition to the current stack frame.
    pub fn define(&mut self, name: impl Into<String>, def: impl Eval) {
        let name = name.into();
        let defmeta = DefineMeta { name: name.clone() };
        self.frame.defs.insert(name, def.into_val().with_meta(|m| m.push(defmeta)));
    }

    /// Remove a definition from the current stack frame, by name.
    pub fn definition_remove(&mut self, name: impl AsRef<str>) {
        self.frame.defs.remove(name.as_ref());
    }

    /// Get all available definitions.
    pub fn all_definitions(&self) -> DefSet {
        let mut defs = self.frame.defs.clone();
        for (name, def) in self.defstacks.iter_latest() {
            defs.insert(name.clone(), def.clone());
        }
        defs
    }

    /// Is the interpreter at the top level? If so, uplevel will fail,
    /// and the remaining children and body parts are all that is left
    /// for the interpreter to interpret before it is replete.
    pub fn is_toplevel(&self) -> bool { self.parents.is_empty() }

    // maybe all of these should be within List
    // and just have stack_ref and stack_mut
    /// Get a reference to the stack
    pub fn stack_ref(&self) -> &List { &self.stack }
    /// Remove and return the top value on the stack if it isn't empty
    pub fn stack_pop_val(&mut self) -> Option<Val> { self.stack.pop() }
    /// Get the top value on the stack if it isn't empty
    pub fn stack_top_val(&self) -> Option<&Val> { self.stack.top() }
    /// Put something on top of the stack
    pub fn stack_push(&mut self, v: impl Into<Val>) { self.stack.push(v.into()); }
    /// Length of the stack :)
    pub fn stack_len(&self) -> usize { self.stack.len() }

    fn resolve_ref(&self, s: impl AsRef<str>) -> Option<&Val> {
        if let Some(def) = self.frame.defs.get(s.as_ref()) {
            Some(def)
        } else if let Some(def) = self.defstacks.get_latest(s) {
            Some(def)
        } else { None }
    }

    /// Grab a reference to the remaining code in the current stack frame.
    pub fn body_mut(&mut self) -> &mut List { &mut self.frame.body }

    /// Returns None on completion or Some(Val) on pause or error.
    pub fn run(&mut self) -> Option<Val> {
        loop {
            match self.frame.childs.pop() {
                Some(ChildFrame::ListFrame(f)) => {
                    self.enter_child_frame(f);
                },
                Some(ChildFrame::PausedFrame(mut f)) => {
                    if let Some(fy) = f.next() {
                        self.frame.childs.push(ChildFrame::PausedFrame(f));
                        if let ret@Some(_) = self.handle_yield(fy) {
                            return ret;
                        }
                    }
                },
                None => {
                    if let Some(next) = self.frame.body.pop() {
                        if let Some(s) = next.downcast_ref::<Symbol>() {
                            self.call(s.clone());
                        } else {
                            self.stack_push(next);
                        }
                    } else if !self.enter_parent_frame() {
                        return None;
                    }
                },
            }
        }
    }

    fn handle_yield(&mut self, y: FrameYield) -> Option<Val> {
        match y {
            FrameYield::Pause(v) => return Some(v),
            FrameYield::Eval(v) => self.handle_eval(v),
            FrameYield::Call(v) => if let r@Some(_) = self.handle_call(v) { return r; },
            FrameYield::StackPush(v) => self.stack_push(v),
            FrameYield::StackGetOp(op) =>
                if let r@Some(_) = self.handle_stackop(op) { return r; },
            FrameYield::StackGetAll(yr) => yr.set(Some(self.stack.clone())),
            FrameYield::Quote(yr) => if let r@Some(_) = self.handle_quote(yr) { return r; },
            FrameYield::Uplevel(v) => if let r@Some(_) = self.handle_uplevel(v) { return r; },
            FrameYield::Define(name, def) => self.define(name, def),
            FrameYield::Definitions(false, yr) => yr.set(Some(self.frame.defs.clone())),
            FrameYield::Definitions(true, yr) => yr.set(Some(self.all_definitions())),
            FrameYield::ResolveDefinition(name, yr) => yr.set(self.resolve_ref(&name).map(Val::clone)),
            FrameYield::GetCallStack(yr) => yr.set(Some(self.call_stack_names())),
        }
        None
    }

    /// Make the interpreter stop doing things,
    /// but leave its toplevel definitions intact.
    pub fn reset(&mut self) {
        while self.enter_parent_frame() {}
        self.frame.childs = vec![];
        self.frame.body = List::default();
    }

    /// Immediately call `name` when the interpreter is next resumed
    pub fn call(&mut self, name: impl Into<Symbol>) {
        let s = name.into();
        self.frame.childs.push((move |mut i: Handle| async move {
            i.call(s).await;
        }).into());
    }

    // maybe this should be a "stack mismatch" (expected stack, actual stack)
    fn handle_stackop(&mut self, mut op: StackGetOp) -> Option<Val> {
        let op_nth = op.pop.unwrap_or(0);
        if op_nth + op.req.len() > self.stack.len() {
            return Some(IsError::add("stack-empty".to_symbol()));
        }
        let mut valiter = 0;
        let mut res = vec![];
        for req in op.req.iter() {
            let val =
                match op.pop {
                    None => self.stack.pop().unwrap(),
                    Some(nth) => {
                        let v = self.stack.get(valiter + nth).unwrap().clone();
                        valiter += 1;
                        v
                    },
                };
            match req {
                StackGetRequest::OfType(t) => {
                    // TODO put values back on stack if pop
                    // and collect these into expected/actual stack lists
                    // and do (stack-mismatch expected actual)
                    if t != val.type_ref() {
                        if op.pop.is_none() {
                            while let Some(v) = res.pop() {
                                self.stack.push(v);
                            }
                        }
                        return Some(IsError::add(List::from(vec![
                            "wrong-type".to_symbol().into(),
                            // (*t).into(),
                            val.into(),
                        ])));
                    }
                },
                StackGetRequest::Any => {},
            }
            res.push(val);
        }
        op.resolve_with(res);
        None
    }

    fn handle_eval(&mut self, f: ChildFrame) {
        self.frame.childs.push(f);
    }

    /// Evaluate the given code.
    /// This is probably what you will use to do stuff with a fresh interpreter.
    pub fn eval_next(&mut self, f: impl EvalOnce) {
        let f = f.into();
        if self.is_complete() {
            match f {
                ChildFrame::ListFrame(mut frame) => {
                    // use frame but keep defs
                    std::mem::swap(&mut self.frame, &mut frame);
                    std::mem::swap(&mut self.frame.defs, &mut frame.defs);
                    if !frame.defs.is_empty() {
                        todo!("eval_next has defs");
                    }
                },
                paused@ChildFrame::PausedFrame(_) => {
                    self.frame.childs.push(paused);
                },
            }
        } else {
            self.handle_eval(f);
        }
    }

    fn handle_call(&mut self, s: Symbol) -> Option<Val> {
        if let Some(def) = self.resolve_ref(&s) {
            let d = def.clone();
            self.frame.childs.push(d.into());
            None
        } else {
            Some(IsError::add(List::from(vec!["undefined".to_symbol().into(), s.into()])))
        }
    }

    fn handle_quote(&mut self, yr: YieldReturn<Val>) -> Option<Val> {
        if let Some(q) = self.frame.body.pop() {
            yr.set(Some(q));
            None
        } else {
            Some(IsError::add("quote-nothing".to_symbol()))
        }
    }

    fn handle_uplevel(&mut self, f: ChildFrame) -> Option<Val> {
        if self.enter_parent_frame() {
            self.frame.childs.push(f);
            None
        } else {
            Some(IsError::add("root-uplevel".to_symbol()))
        }
    }

    fn enter_child_frame(&mut self, mut frame: ListFrame) {
        std::mem::swap(&mut self.frame, &mut frame);
        self.defstacks.push(&frame.defs);
        self.parents.push(frame);
    }

    fn enter_parent_frame(&mut self) -> bool {
        if let Some(mut frame) = self.parents.pop() {
            std::mem::swap(&mut self.frame, &mut frame);

            self.defstacks.pop(&self.frame.defs);

            if !frame.body.is_empty() {
                self.frame.childs.push(ChildFrame::ListFrame(frame));
            }
            true
        } else { false }
    }

    // basic look at all the ListFrame and see
    fn call_stack_names(&self) -> Vec<Option<String>> {
        let mut r = vec![];
        if let Some(DefineMeta { name }) = self.frame.meta.first_ref::<DefineMeta>() {
            r.push(Some(name.clone()));
        } else {
            r.push(None);
        }

        for p in self.parents.iter().rev() {
            if let Some(DefineMeta { name }) = p.meta.first_ref::<DefineMeta>() {
                r.push(Some(name.clone()));
            } else {
                r.push(None);
            }
        }

        r
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_interp(l: Vec<Val>) -> Interpreter {
        let mut i = Interpreter::default();
        i.eval_next(Val::from(List::from(l)));
        i
    }

    #[test]
    fn interp_basic() {
        // empty
        assert_eq!(Interpreter::default().run(), None);
        // stack
        let mut i = new_interp(vec![7.into()]);
        assert_eq!(i.stack_pop_val(), None);
        assert_eq!(i.run(), None);
        assert_eq!(i.stack_pop_val(), Some(7.into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    async fn toplevel_def(mut i: Handle) {
        i.stack_push("yay".to_string()).await;
    }

    #[test]
    fn interp_def() {
        let mut i =
            new_interp(vec![
                "thingy".to_symbol().into(),
                "test".to_symbol().into(),
            ]);
        i.define("test", |mut i: Handle| async move {
            i.stack_push("hello".to_string()).await;
        });
        i.define("thingy", toplevel_def);
        assert_eq!(i.run(), None);
        assert_eq!(i.stack_pop_val(), Some(String::from("hello").into()));
        assert_eq!(i.stack_pop_val(), Some(String::from("yay").into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_quote() {
        let mut i =
            new_interp(vec![
                "quote".to_symbol().into(),
                "egg".to_symbol().into(),
            ]);
        i.define("quote", |mut i: Handle| async move {
            let q = i.quote_val().await;
            i.stack_push(q).await;
        });
        assert_eq!(i.run(), None);
        assert_eq!(i.stack_pop_val(), Some("egg".to_symbol().into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_uplevel() {
        let mut i =
            new_interp(vec![
                "thing".to_symbol().into(),
                "egg".to_symbol().into(),
            ]);
        i.define("thing", Val::from(List::from(vec![ "upquote".to_symbol().into() ])));
        i.define("upquote", |mut i: Handle| async move {
            i.uplevel(|mut i: Handle| async move {
                let q = i.quote_val().await;
                i.stack_push(q).await;
            }).await;
        });
        assert_eq!(i.run(), None);
        assert_eq!(i.stack_pop_val(), Some("egg".to_symbol().into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_uplevel_closure() {
        let mut i =
            new_interp(vec![
                "thing".to_symbol().into(),
            ]);
        i.define("thing", Val::from(List::from(vec![ "upfive".to_symbol().into() ])));
        i.define("upfive", |mut i: Handle| async move {
            let five = "five".to_symbol();
            i.uplevel(move |mut i: Handle| async move {
                i.stack_push(five).await;
            }).await;
        });
        assert_eq!(i.run(), None);
        assert_eq!(i.stack_pop_val(), Some("five".to_symbol().into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_uplevel2() {
        let mut i =
            new_interp(vec![
                "thing1".to_symbol().into(),
                "egg".to_symbol().into(),
            ]);
        i.define("thing1", Val::from(List::from(vec![ "thing2".to_symbol().into() ])));
        i.define("thing2", Val::from(List::from(vec![ "upquote2".to_symbol().into() ])));
        i.define("upquote2", |mut i: Handle| async move {
            i.uplevel(move |mut i: Handle| async move {
                i.uplevel(move |mut i: Handle| async move {
                    let q = i.quote_val().await;
                    i.stack_push(q).await;
                }).await;
            }).await;
        });
        assert_eq!(i.run(), None);
        assert_eq!(i.stack_pop_val(), Some("egg".to_symbol().into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_eval() {
        let mut i =
            new_interp(vec![
                "eval".to_symbol().into(),
            ]);
        i.define("eval", |mut i: Handle| async move {
            i.eval(Val::from(List::from(vec![ "inner".to_symbol().into() ]))).await;
        });
        i.define("inner", |mut i: Handle| async move {
            i.eval(|mut i: Handle| async move {
                i.stack_push(5).await;
            }).await;
        });
        assert_eq!(i.run(), None);
        assert_eq!(i.stack_pop_val(), Some(5.into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_eval_closure() {
        let mut i =
            new_interp(vec![
                "five".to_symbol().into(),
            ]);
        i.define("five", |mut i: Handle| async move {
            let five = "five".to_symbol();
            i.eval(move |mut i: Handle| async move {
                i.stack_push(five).await;
            }).await;
        });
        assert_eq!(i.run(), None);
        assert_eq!(i.stack_pop_val(), Some("five".to_symbol().into()));
        assert_eq!(i.stack_pop_val(), None);
    }

}

