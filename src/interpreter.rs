
//! An [Interpreter] for Worst code.

use crate::base::*;

mod base;
mod handle;
mod defset;
pub use base::{Handle, DefineMeta, Builtin};
pub use defset::DefSet;
use base::*;
pub use handle::*;

/// A Worst interpreter, the thing you define functions for and run code in and stuff.
#[derive(Default)]
pub struct Interpreter {
    frame: ListFrame,
    parents: Vec<ListFrame>,
    stack: List,
}
impl Value for Interpreter {}

impl Interpreter {

    /// Check if there is anything else left to evaluate.
    pub fn is_complete(&self) -> bool {
        self.frame.is_empty() && self.parents.is_empty()
    }

    /// Add a definition into the environment for the current stack frame.
    /// This is different to [define] as it will not be removable with
    /// [definition_remove] later.
    // ... but that butld change
    pub fn add_definitions(&mut self, defs: &DefSet) {
        self.frame.defenv.append(defs);
    }

    /// Insert a value, as-is, as a definition in the current stack frame.
    pub fn add_definition(&mut self, name: impl Into<String>, def: impl Into<Val>) {
        self.frame.add_definition(name, def);
    }

    /// Add a definition to the current stack frame.
    /// Inserts meta values such as name and a static environment.
    pub fn define(&mut self, name: impl Into<String>, def: impl Eval) {
        let name = name.into();
        let meta = def.eval_meta();
        let mut def = def.into_val();
        if meta {
            let m = def.meta_mut();
            m.push(DefineMeta { name: Some(name.clone()) });
            m.push(self.all_definitions());
        }
        self.add_definition(name, def);
    }

    /// Remove a definition from the current stack frame, by name,
    /// and return its previous value if there was one.
    pub fn definition_remove(&mut self, name: impl AsRef<str>) -> Option<Val> {
        self.frame.remove_local(name)
    }

    /// Get all local definitions (the ones defined in this stack frame,
    /// not including ambient definitions).
    pub fn local_definitions(&self) -> DefSet {
        self.frame.locals.clone()
    }

    /// Get all available definitions.
    pub fn all_definitions(&self) -> DefSet {
        self.frame.all_defs()
    }

    /// Is the interpreter at the top level? If so, uplevel will fail,
    /// and the remaining children and body parts are all that is left
    /// for the interpreter to interpret before it is replete.
    pub fn is_toplevel(&self) -> bool { self.parents.is_empty() }

    // maybe all of these should be within List
    // and just have stack_ref and stack_mut
    /// Get a reference to the stack
    pub fn stack_ref(&self) -> &List { &self.stack }
    /// Get a mutable reference to the stack
    pub fn stack_ref_mut(&mut self) -> &mut List { &mut self.stack }
    /// Put something on top of the stack
    pub fn stack_push(&mut self, v: impl Into<Val>) { self.stack.push(v.into()); }
    /// Pop the top thing off the stack
    pub fn stack_pop(&mut self) -> Option<Val> { self.stack.pop() }

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

    fn handle_eval_once(&mut self, v: ToEvalOnce) {
        let child = self.frame.eval_once(v);
        self.frame.childs.push(child);
    }

    fn handle_definitions(&self, all: bool) -> DefSet {
        if all {
            self.frame.all_defs()
        } else {
            self.frame.locals.clone()
        }
    }
    fn handle_get_definition(&self, name: &str, resolver: ResolveDefinition) -> Option<Val> {
        if let Some(def) = self.frame.resolve_definition(name, &resolver).cloned() {
            return Some(def);
        }

        if resolver.recursive_dynamic {
            for f in self.parents.iter().rev() {
                if let Some(def) = f.resolve_definition(name, &resolver).cloned() {
                    return Some(def);
                }
            }
        }

        None
    }

    fn handle_yield(&mut self, y: FrameYield) -> Option<Val> {
        match y {
            FrameYield::Pause(v) => return Some(v),
            FrameYield::Eval(v) => self.handle_eval_once(v),
            FrameYield::Call(v) => if let r@Some(_) = self.handle_call(v) { return r; },
            FrameYield::StackPush(v) => self.stack_push(v),
            FrameYield::StackPop(yr) => yr.set(self.stack_ref_mut().pop()),
            FrameYield::StackGetAll(yr) => yr.set(Some(self.stack.clone())),
            FrameYield::Quote(yr) => if let r@Some(_) = self.handle_quote(yr) { return r; },
            FrameYield::Uplevel(v) => if let r@Some(_) = self.handle_uplevel(v) { return r; },
            FrameYield::AddDefinition { name, def } =>
                self.frame.add_definition(name, def),
            FrameYield::Definitions { all, ret } =>
                ret.set(Some(self.handle_definitions(all))),
            FrameYield::GetDefinition { name, resolver, ret } =>
                ret.set(self.handle_get_definition(name.as_ref(), resolver)),
            FrameYield::RemoveDefinition { name, ret } =>
                ret.set(self.frame.locals.remove(name)),
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
        let child =
            self.frame.eval_once((move |mut i: Handle| async move {
                i.call(s).await;
            }).into_eval_once());
        self.frame.childs.push(child);
    }

    /// Evaluate the given code.
    /// This is probably what you will use to do stuff with a fresh interpreter.
    pub fn eval_next(&mut self, f: impl EvalOnce) {
        let f = f.into_eval_once();
        if self.is_complete() {
            match self.frame.eval_once(f) {
                ChildFrame::ListFrame(mut frame) => {
                    // use frame but keep defs
                    std::mem::swap(&mut self.frame, &mut frame);
                    std::mem::swap(&mut self.frame.locals, &mut frame.locals);
                    if !frame.locals.is_empty() {
                        todo!("eval_next has defs");
                    }
                },
                paused@ChildFrame::PausedFrame(_) => {
                    self.frame.childs.push(paused);
                },
            }
        } else {
            self.handle_eval_once(f);
        }
    }

    fn handle_call(&mut self, s: Symbol) -> Option<Val> {
        if let Some(def) = self.frame.find_def(&s) {
            let d = def.clone();
            let child = self.frame.eval_once(d.into_eval_once());
            self.frame.childs.push(child);
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

    fn handle_uplevel(&mut self, f: ToEvalOnce) -> Option<Val> {
        if self.enter_parent_frame() {
            let child = self.frame.eval_once(f);
            self.frame.childs.push(child);
            None
        } else {
            Some(IsError::add("root-uplevel".to_symbol()))
        }
    }

    fn enter_child_frame(&mut self, mut frame: ListFrame) {
        std::mem::swap(&mut self.frame, &mut frame);
        self.parents.push(frame);
    }

    fn enter_parent_frame(&mut self) -> bool {
        if let Some(mut frame) = self.parents.pop() {
            std::mem::swap(&mut self.frame, &mut frame);

            if !frame.is_empty() {
                self.frame.childs.push(ChildFrame::ListFrame(frame));
            }
            true
        } else { false }
    }

    // basic look at all the ListFrame and see
    fn call_stack_names(&self) -> Vec<Option<String>> {
        // dbg!(&self.frame);
        let mut r = vec![];
        r.push(self.frame.def_name().map(String::from));
        for p in self.parents.iter().rev() {
            r.push(p.def_name().map(String::from));
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
        assert!(Interpreter::default().run().is_none());
        // stack
        let mut i = new_interp(vec![7.into()]);
        assert!(i.stack_ref().is_empty());
        assert!(i.run().is_none());
        assert_eq!(i.stack_pop().and_then(Val::downcast::<i64>), Some(7 as i64));
        assert!(i.stack_ref().is_empty());
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
        assert!(i.run().is_none());
        assert_eq!(i.stack_pop().and_then(Val::downcast::<String>), Some(String::from("hello")));
        assert_eq!(i.stack_pop().and_then(Val::downcast::<String>), Some(String::from("yay")));
        assert!(i.stack_ref().is_empty());
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
        assert!(i.run().is_none());
        assert_eq!(i.stack_pop().and_then(Val::downcast::<Symbol>), Some("egg".to_symbol()));
        assert!(i.stack_ref().is_empty());
    }

    #[test]
    fn test_uplevel() {
        let mut i =
            new_interp(vec![
                "thing".to_symbol().into(),
                "egg".to_symbol().into(),
            ]);
        i.define("upquote", |mut i: Handle| async move {
            i.uplevel(|mut i: Handle| async move {
                let q = i.quote_val().await;
                i.stack_push(q).await;
            }).await;
        });
        i.define("thing", Val::from(List::from(vec![ "upquote".to_symbol().into() ])));
        assert!(i.run().is_none());
        assert_eq!(i.stack_pop().and_then(Val::downcast::<Symbol>), Some("egg".to_symbol()));
        assert!(i.stack_ref().is_empty());
    }

    #[test]
    fn test_uplevel_closure() {
        let mut i =
            new_interp(vec![
                "thing".to_symbol().into(),
            ]);
        i.define("upfive", |mut i: Handle| async move {
            let five = "five".to_symbol();
            i.uplevel(move |mut i: Handle| async move {
                i.stack_push(five).await;
            }).await;
        });
        i.define("thing", Val::from(List::from(vec![ "upfive".to_symbol().into() ])));
        assert!(i.run().is_none());
        assert_eq!(i.stack_pop().and_then(Val::downcast::<Symbol>), Some("five".to_symbol()));
        assert!(i.stack_ref().is_empty());
    }

    #[test]
    fn test_uplevel2() {
        let mut i =
            new_interp(vec![
                "thing1".to_symbol().into(),
                "egg".to_symbol().into(),
            ]);
        i.define("upquote2", |mut i: Handle| async move {
            i.uplevel(move |mut i: Handle| async move {
                i.uplevel(move |mut i: Handle| async move {
                    let q = i.quote_val().await;
                    i.stack_push(q).await;
                }).await;
            }).await;
        });
        i.define("thing2", Val::from(List::from(vec![ "upquote2".to_symbol().into() ])));
        i.define("thing1", Val::from(List::from(vec![ "thing2".to_symbol().into() ])));
        assert!(i.run().is_none());
        assert_eq!(i.stack_pop().and_then(Val::downcast::<Symbol>), Some("egg".to_symbol()));
        assert!(i.stack_ref().is_empty());
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
        assert!(i.run().is_none());
        assert_eq!(i.stack_pop().and_then(Val::downcast::<i64>), Some(5 as i64));
        assert!(i.stack_ref().is_empty());
    }

    #[test]
    fn test_eval_uplevel() {
        let mut i = new_interp(vec![ "eval".to_symbol().into(), ]);
        i.define("uplevel", |mut i: Handle| async move {
            let v = i.stack_pop_val().await;
            i.uplevel(v).await;
        });
        i.define("one", |mut i: Handle| async move {
            i.stack_push(1).await;
        });
        i.define("eval-inner", |mut i: Handle| async move {
            i.stack_push("one".to_symbol()).await;
            i.stack_push("uplevel".to_symbol()).await;
            i.eval(List::from(vec![ "uplevel".to_symbol().into() ])).await;
            i.stack_push(2).await;
        });
        i.define("eval", Val::from(List::from(vec![
            "eval-inner".to_symbol().into(),
            3.into()
        ])));
        assert!(i.run().is_none());
        assert_eq!(i.stack_pop().and_then(Val::downcast::<i64>), Some(3 as i64));
        assert_eq!(i.stack_pop().and_then(Val::downcast::<i64>), Some(2 as i64));
        assert_eq!(i.stack_pop().and_then(Val::downcast::<i64>), Some(1 as i64));
        assert!(i.stack_ref().is_empty());
    }

    #[test]
    fn test_eval_pre_uplevel1() {
        let mut i = new_interp(vec![ "eval".to_symbol().into(), ]);
        i.define("uplevel", |mut i: Handle| async move {
            let v = i.stack_pop_val().await;
            i.uplevel(v).await;
        });
        i.define("one", |mut i: Handle| async move {
            i.stack_push(1).await;
        });
        i.define("eval", |mut i: Handle| async move {
            i.eval_child(List::default(), |mut i: Handle| async move {
                i.stack_push("one".to_symbol()).await;
                i.eval(List::from(vec![ "uplevel".to_symbol().into() ])).await;
                i.stack_push(2).await;
            }).await;
            i.call_stack_names().await;
            i.stack_push(3).await;
        });
        assert!(i.run().is_none());
        assert_eq!(i.stack_pop().and_then(Val::downcast::<i64>), Some(3 as i64));
        assert_eq!(i.stack_pop().and_then(Val::downcast::<i64>), Some(2 as i64));
        assert_eq!(i.stack_pop().and_then(Val::downcast::<i64>), Some(1 as i64));
        assert!(i.stack_ref().is_empty());
    }

    #[test]
    fn test_eval_pre_uplevel2() {
        let mut i = new_interp(vec![ "eval".to_symbol().into(), ]);
        i.define("uplevel", |mut i: Handle| async move {
            let v = i.stack_pop_val().await;
            i.uplevel(v).await;
        });
        i.define("one", |mut i: Handle| async move {
            i.stack_push(1).await;
        });
        i.define("eval", |mut i: Handle| async move {
            i.eval_child(List::default(), |mut i: Handle| async move {
                i.stack_push("one".to_symbol()).await;
                i.stack_push("uplevel".to_symbol()).await;
                i.eval(List::from(vec![ "uplevel".to_symbol().into() ])).await;
                i.stack_push(2).await;
            }).await;
            i.call_stack_names().await;
            i.stack_push(3).await;
        });
        assert!(i.run().is_none());
        assert_eq!(i.stack_pop().and_then(Val::downcast::<i64>), Some(3 as i64));
        assert_eq!(i.stack_pop().and_then(Val::downcast::<i64>), Some(2 as i64));
        assert_eq!(i.stack_pop().and_then(Val::downcast::<i64>), Some(1 as i64));
        assert!(i.stack_ref().is_empty());
    }

    #[test]
    fn test_eval_pre_uplevel3() {
        let mut i = new_interp(vec![ "eval".to_symbol().into(), ]);
        i.define("noop", Val::from(List::default()));
        i.define("uplevel", |mut i: Handle| async move {
            let v = i.stack_pop_val().await;
            i.uplevel(v).await;
        });
        i.define("one", |mut i: Handle| async move {
            i.stack_push(1).await;
        });
        i.define("eval", |mut i: Handle| async move {
            i.eval_child(List::from(vec!["noop".to_symbol().into()]), |mut i: Handle| async move {
                i.stack_push("one".to_symbol()).await;
                i.stack_push("uplevel".to_symbol()).await;
                i.eval(List::from(vec![ "uplevel".to_symbol().into() ])).await;
                i.stack_push(2).await;
            }).await;
            i.call_stack_names().await;
            i.stack_push(3).await;
        });
        assert!(i.run().is_none());
        assert_eq!(i.stack_pop().and_then(Val::downcast::<i64>), Some(3 as i64));
        assert_eq!(i.stack_pop().and_then(Val::downcast::<i64>), Some(2 as i64));
        assert_eq!(i.stack_pop().and_then(Val::downcast::<i64>), Some(1 as i64));
        assert!(i.stack_ref().is_empty());
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
        assert!(i.run().is_none());
        assert_eq!(i.stack_pop().and_then(Val::downcast::<Symbol>), Some("five".to_symbol()));
        assert!(i.stack_ref().is_empty());
    }

}

