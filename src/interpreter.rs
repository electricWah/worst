
use std::cell::Cell;
use std::rc::Rc;
use core::pin::Pin;
use core::future::Future;
use std::collections::HashMap;
use genawaiter::{ rc::Gen, rc::Co };

use crate::base::*;
use crate::list::List;

type YieldReturn<T> = Rc<Cell<Option<T>>>;

// Don't want to leak ChildFrame required by Eval/EvalOnce
mod private {
    use super::*;

    #[derive(Default)]
    pub struct ListFrame {
        pub childs: List<ChildFrame>,
        pub body: List<Val>,
        pub defs: HashMap<String, Definition>,
    }
    impl ListFrame {
        pub fn new_body(body: List<Val>) -> Self {
            ListFrame { body, ..ListFrame::default() }
        }
    }

    pub enum FrameYield {
        Pause,
        Eval(ChildFrame),
        Call(Symbol),
        StackPush(Val),
        StackPop(YieldReturn<Val>),
        StackGet(YieldReturn<List<Val>>),
        Quote(YieldReturn<Val>),
        Uplevel(ChildFrame),
        Define(String, Definition),
    }

    pub struct PausedFrame {
        pub body: Box<dyn Iterator<Item=FrameYield>>,
    }

    pub enum ChildFrame {
        ListFrame(ListFrame),
        PausedFrame(PausedFrame),
    }
    pub trait IntoChildFrame: Into<ChildFrame> {}
}
use private::*;

pub struct Handle {
    co: Co<FrameYield>,
}
// not sure if these all have to be mut
impl Handle {
    pub async fn pause(&mut self) {
        self.co.yield_(FrameYield::Pause).await;
    }
    pub async fn eval(&mut self, f: impl EvalOnce) {
        self.co.yield_(FrameYield::Eval(f.into())).await;
    }
    pub async fn eval_child(&mut self, body: List<Val>, child: impl EvalOnce) {
        let mut frame = ListFrame::new_body(body);
        frame.childs.push(child.into());
        self.co.yield_(FrameYield::Eval(ChildFrame::ListFrame(frame))).await;
    }
    pub async fn call(&mut self, s: impl Into<Symbol>) {
        self.co.yield_(FrameYield::Call(s.into())).await;
    }
    pub async fn stack_push(&mut self, v: impl Into<Val>) {
        self.co.yield_(FrameYield::StackPush(v.into())).await;
    }
    pub async fn stack_pop_val(&mut self) -> Option<Val> {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::StackPop(Rc::clone(&r))).await;
        r.take()
    }
    pub async fn stack_pop<T: Value>(&mut self) -> Option<T> {
        match self.stack_pop_val().await {
            Some(v) =>
                match v.downcast::<T>() {
                    Ok(r) => Some(*r),
                    Err(e) => {
                        self.stack_push(e).await;
                        None
                    },
                },
            None => None,
        }
    }
    pub async fn stack_get(&mut self) -> List<Val> {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::StackGet(Rc::clone(&r))).await;
        r.take().unwrap()
    }
    pub async fn quote(&mut self) -> Option<Val> {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::Quote(Rc::clone(&r))).await;
        r.take()
    }
    pub async fn uplevel(&mut self, f: impl EvalOnce) {
        self.co.yield_(FrameYield::Uplevel(f.into())).await;
    }
    pub async fn define(&mut self, name: impl Into<String>, def: impl Eval) {
        self.co.yield_(FrameYield::Define(name.into(), def.into())).await;
    }
}

// Rust code function

pub trait Eval: IntoChildFrame + Into<Definition> {}
pub trait EvalOnce: IntoChildFrame {}

impl<T: Into<ChildFrame>> IntoChildFrame for T {}

type BuiltinFnRet = Pin<Box<dyn Future<Output = ()> + 'static>>;

#[derive(Clone)]
pub struct Builtin(Rc<dyn Fn(Handle) -> BuiltinFnRet>);

impl std::fmt::Debug for Builtin {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl<F: 'static + Future<Output=()>,
     T: 'static + Eval + Fn(Handle) -> F>
        From<T> for Builtin {
    fn from(f: T) -> Self {
        Builtin(Rc::new(move |i: Handle| { Box::pin(f(i)) }))
    }
}
impl PartialEq for Builtin {
    fn eq(&self, that: &Self) -> bool { Rc::ptr_eq(&self.0, &that.0) }
}
impl Eq for Builtin {}
impl ImplValue for Builtin {}

impl<F: 'static + Future<Output=()>,
     T: 'static + Fn(Handle) -> F>
     Eval for T {}

impl<F: 'static + Future<Output=()>,
     T: 'static + FnOnce(Handle) -> F>
     EvalOnce for T {}

impl Eval for List<Val> {}
impl EvalOnce for List<Val> {}
impl Eval for Builtin {}
impl EvalOnce for Builtin {}
impl Eval for Definition {}

// Code frame with a body being an in-progress Rust function
impl PausedFrame {
    fn next(&mut self) -> Option<FrameYield> {
        self.body.next()
    }
}

impl From<List<Val>> for ChildFrame {
    fn from(body: List<Val>) -> Self {
        ChildFrame::ListFrame(ListFrame::new_body(body))
    }
}

impl<F: 'static + Future<Output=()>,
     T: 'static + FnOnce(Handle) -> F>
        From<T> for ChildFrame {
    fn from(f: T) -> Self {
        ChildFrame::PausedFrame(PausedFrame {
            body: Box::new(Gen::new(move |co| async move {
                f(Handle { co }).await;
            }).into_iter()),
        })
    }
}
impl From<Builtin> for ChildFrame {
    fn from(b: Builtin) -> Self {
        ChildFrame::PausedFrame(PausedFrame {
            body: Box::new(Gen::new(move |co| async move {
                b.0(Handle { co }).await;
            }).into_iter()),
        })
    }
}

#[derive(Debug, Clone)]
pub enum Definition {
    List(List<Val>),
    Builtin(Builtin),
}
impl From<List<Val>> for Definition {
    fn from(v: List<Val>) -> Self { Definition::List(v) }
}
impl<T: Into<Builtin>> From<T> for Definition {
    fn from(v: T) -> Self { Definition::Builtin(v.into()) }
}

impl From<Definition> for ChildFrame {
    fn from(def: Definition) -> Self {
        match def {
            Definition::List(body) => body.into(),
            Definition::Builtin(b) => b.into(),
        }
    }
}

#[derive(Default)]
pub struct Paused {
    frame: ListFrame,
    parents: List<ListFrame>,
    stack: List<Val>,
    defstacks: HashMap<String, List<Definition>>,
}

impl Paused {

    // pub fn eval(&mut self, f: impl EvalOnce) {
    //     self.handle_eval(f.into());
    //     while !self.run() {}
    // }

    fn new(body: impl Into<List<Val>>) -> Self {
        Paused {
            frame: ListFrame::new_body(body.into()),
            ..Paused::default()
        }
    }

    pub fn define(&mut self, name: impl Into<String>, def: impl Eval) {
        self.frame.defs.insert(name.into(), def.into());
    }
    fn add_def(&mut self, name: impl Into<String>, def: Definition) {
        self.frame.defs.insert(name.into(), def);
    }

    // pub fn definition_get(&self, name: impl AsRef<str>) -> Option<&Definition> {
    // }

    pub fn definition_remove(&mut self, name: impl AsRef<String>) {
        self.frame.defs.remove(name.as_ref());
    }

    // pub fn set_body(&mut self, body: List<Val>) { self.frame.body = body; }
    // fn body_ref(&self) -> &List<Val> { &self.frame.body }

    pub fn is_toplevel(&self) -> bool { self.parents.is_empty() }

    // pub fn reset(&mut self) // in Paused only
    // maybe not needed with like, eval_in_new_body?
    // pub fn enter_new_frame(&mut self, body: List<Val>)
    // pub fn definitions()
    // pub fn all_definitions()
    // pub fn error(name, ...) // no?
    // pub fn try_resolve(name)
    // pub fn stack_ref(i, ty)
    // pub fn stack_get()
    // pub fn stack_set(l)

    // also stack_pop::<T>() -> Option<T>
    // also stack_pop_purpose() etc etc
    // maybe stack() + stack_mut() -> StackRef
    // with pop_any() and pop::<T> and ref etc?
    pub fn stack_pop_val(&mut self) -> Option<Val> { self.stack.pop() }
    pub fn stack_push(&mut self, v: impl Into<Val>) { self.stack.push(v.into()); }
    pub fn stack_len(&self) -> usize { self.stack.len() }

    fn resolve_ref(&self, s: &Symbol) -> Option<&Definition> {
        if let Some(def) = self.frame.defs.get(s.as_string()) {
            Some(def)
        } else if let Some(defstack) = self.defstacks.get(s.as_string()) {
            if let Some(def) = defstack.top() {
                Some(def)
            } else { None }
        } else { None }
    }

    fn read_body(&mut self) -> Option<Val> { self.frame.body.pop() }

    fn run(&mut self) -> bool {
        loop {
            match self.frame.childs.pop() {
                Some(ChildFrame::ListFrame(f)) => {
                    self.enter_child_frame(f);
                },
                Some(ChildFrame::PausedFrame(mut f)) => {
                    if let Some(fy) = f.next() {
                        self.frame.childs.push(ChildFrame::PausedFrame(f));
                        if self.handle_yield(fy) { return false; }
                    }
                },
                None => {
                    if let Some(next) = self.read_body() {
                        if let Some(s) = next.downcast_ref::<Symbol>() {
                            self.create_call(s.clone());
                        } else {
                            self.stack.push(next);
                        }
                    } else {
                        if !self.enter_parent_frame() { return true; }
                    }
                },
            }
        }
    }

    // only used from Handle
    // return whether to pause evaluation
    fn handle_yield(&mut self, y: FrameYield) -> bool {
        match y {
            FrameYield::Pause => return true,
            FrameYield::Eval(v) => self.handle_eval(v),
            FrameYield::Call(v) => if !self.handle_call(v) { return true; },
            FrameYield::StackPush(v) => self.stack_push(v),
            FrameYield::StackPop(yr) => yr.set(self.stack_pop_val()),
            FrameYield::StackGet(yr) => yr.set(Some(self.stack.clone())),
            FrameYield::Quote(yr) => self.handle_quote(yr),
            FrameYield::Uplevel(v) => if !self.handle_uplevel(v) { return true; },
            FrameYield::Define(name, def) => self.add_def(name, def),
        }
        false
    }

    fn create_call(&mut self, s: Symbol) {
        self.frame.childs.push((move |mut i: Handle| async move {
            i.call(s).await;
        }).into());
    }

    fn handle_eval(&mut self, f: ChildFrame) {
        self.frame.childs.push(f);
    }

    // maybe Result<(), Val>
    fn handle_call(&mut self, s: Symbol) -> bool {
        if let Some(def) = self.resolve_ref(&s) {
            let d = def.clone();
            self.frame.childs.push(d.into());
            true
        } else {
            self.stack_push(List::from(vec!["undefined".to_symbol().into(), s.into()]));
            false
        }
    }

    fn handle_quote(&mut self, yr: YieldReturn<Val>) {
        yr.set(self.frame.body.pop());
    }

    fn handle_uplevel(&mut self, f: ChildFrame) -> bool {
        if !self.enter_parent_frame() { return false; }
        self.frame.childs.push(f);
        true
    }

    fn enter_child_frame(&mut self, mut frame: ListFrame) {
        std::mem::swap(&mut self.frame, &mut frame);
        for (name, def) in &frame.defs {
            self.defstacks.entry(name.clone()).or_default().push(def.clone());
        }
        self.parents.push(frame);
    }

    fn enter_parent_frame(&mut self) -> bool {
        if let Some(mut frame) = self.parents.pop() {
            std::mem::swap(&mut self.frame, &mut frame);

            for name in self.frame.defs.keys() {
                let _ = self.defstacks.get_mut(name).and_then(|x| x.pop());
                if self.defstacks.get(name).map_or(false, |x| x.is_empty()) {
                    self.defstacks.remove(name);
                }
            }

            if !frame.body.is_empty() {
                self.frame.childs.push(ChildFrame::ListFrame(frame));
            }
            true
        } else { false }
    }
}

#[derive(Default)]
pub struct Builder {
    stack: List<Val>,
    defs: HashMap<String, Definition>,
}

impl Builder {
    pub fn eval(&self, f: impl EvalOnce) -> Result<(), Paused> {
        match f.into() {
            ChildFrame::ListFrame(frame) => {
                let mut i = Paused { frame, ..Paused::default() };
                for (k, v) in self.defs.iter() { i.add_def(k, v.clone()); }
                if i.run() { Ok(()) } else { Err(i) }
            },
            paused@ChildFrame::PausedFrame(_) => {
                let mut i = Paused::new(vec![]);
                for (k, v) in self.defs.iter() { i.add_def(k, v.clone()); }
                i.handle_eval(paused);
                if i.run() { Ok(()) } else { Err(i) }
            },
        }
    }
    pub fn with_stack(mut self, s: impl Into<List<Val>>) -> Self {
        self.stack = s.into();
        self
    }
    pub fn define(mut self, name: impl Into<String>, def: impl Eval) -> Self {
        self.defs.insert(name.into(), def.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interp_basic() {
        // empty
        assert!(Builder::default().eval(List::from(vec![])).is_ok());
        // stack
        let mut i = Paused::new(vec![7.into()]);
        assert_eq!(i.stack_pop_val(), None);
        assert_eq!(i.run(), true);
        assert_eq!(i.stack_pop_val(), Some(7.into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    async fn toplevel_def(mut i: Handle) {
        i.stack_push("yay").await;
    }

    #[test]
    fn interp_def() {
        let mut i =
            Paused::new(vec![
                "thingy".to_symbol().into(),
                "test".to_symbol().into(),
            ]);
        i.define("test", |mut i: Handle| async move {
            i.stack_push("hello").await;
        });
        i.define("thingy", toplevel_def);
        assert_eq!(i.run(), true);
        assert_eq!(i.stack_pop_val(), Some(String::from("hello").into()));
        assert_eq!(i.stack_pop_val(), Some(String::from("yay").into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_quote() {
        let mut i =
            Paused::new(vec![
                "quote".to_symbol().into(),
                "egg".to_symbol().into(),
            ]);
        i.define("quote", |mut i: Handle| async move {
            if let Some(q) = i.quote().await {
                i.stack_push(q).await;
            }
        });
        assert_eq!(i.run(), true);
        assert_eq!(i.stack_pop_val(), Some("egg".to_symbol().into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_uplevel() {
        let mut i =
            Paused::new(vec![
                "thing".to_symbol().into(),
                "egg".to_symbol().into(),
            ]);
        i.define("thing", List::from(vec![ "upquote".to_symbol().into() ]));
        i.define("upquote", |mut i: Handle| async move {
            i.uplevel(|mut i: Handle| async move {
                if let Some(q) = i.quote().await {
                    i.stack_push(q).await;
                }
            }).await;
        });
        assert_eq!(i.run(), true);
        assert_eq!(i.stack_pop_val(), Some("egg".to_symbol().into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_uplevel_closure() {
        let mut i =
            Paused::new(vec![
                "thing".to_symbol().into(),
            ]);
        i.define("thing", List::from(vec![ "upfive".to_symbol().into() ]));
        i.define("upfive", |mut i: Handle| async move {
            let five = "five".to_symbol();
            i.uplevel(move |mut i: Handle| async move {
                i.stack_push(five).await;
            }).await;
        });
        assert_eq!(i.run(), true);
        assert_eq!(i.stack_pop_val(), Some("five".to_symbol().into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_uplevel2() {
        let mut i =
            Paused::new(vec![
                "thing1".to_symbol().into(),
                "egg".to_symbol().into(),
            ]);
        i.define("thing1", List::from(vec![ "thing2".to_symbol().into() ]));
        i.define("thing2", List::from(vec![ "upquote2".to_symbol().into() ]));
        i.define("upquote2", |mut i: Handle| async move {
            i.uplevel(move |mut i: Handle| async move {
                i.uplevel(move |mut i: Handle| async move {
                    if let Some(q) = i.quote().await {
                        i.stack_push(q).await;
                    }
                }).await;
            }).await;
        });
        assert_eq!(i.run(), true);
        assert_eq!(i.stack_pop_val(), Some("egg".to_symbol().into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_eval() {
        let mut i =
            Paused::new(vec![
                "eval".to_symbol().into(),
            ]);
        i.define("eval", |mut i: Handle| async move {
            i.eval(List::from(vec![ "inner".to_symbol().into() ])).await;
        });
        i.define("inner", |mut i: Handle| async move {
            i.eval(|mut i: Handle| async move {
                i.stack_push(5).await;
            }).await;
        });
        assert_eq!(i.run(), true);
        assert_eq!(i.stack_pop_val(), Some(5.into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_eval_closure() {
        let mut i =
            Paused::new(vec![
                "five".to_symbol().into(),
            ]);
        i.define("five", |mut i: Handle| async move {
            let five = "five".to_symbol();
            i.eval(move |mut i: Handle| async move {
                i.stack_push(five).await;
            }).await;
        });
        assert_eq!(i.run(), true);
        assert_eq!(i.stack_pop_val(), Some("five".to_symbol().into()));
        assert_eq!(i.stack_pop_val(), None);
    }

}

