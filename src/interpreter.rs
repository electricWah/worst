
use std::cell::Cell;
use std::rc::Rc;
use core::pin::Pin;
use core::future::Future;
use std::collections::HashMap;
use genawaiter::{ rc::Gen, rc::Co };

use crate::base::*;
use crate::list::List;

// Code frame with a body being a list of data
#[derive(Default)]
struct ListFrame {
    childs: Stack<ChildFrame>,
    body: List<Val>,
    defs: HashMap<String, Definition>,
}

type YieldReturn<T> = Rc<Cell<Option<T>>>;

enum FrameYield {
    Pause,
    ChildFrame(ChildFrame),
    Eval(ChildFrame),
    Call(Symbol),
    StackPush(Val),
    Quote(YieldReturn<Val>),
    Uplevel(ChildFrame),
}

struct InterpHandle {
    co: Co<FrameYield>,
}
// not sure if these all have to be mut
impl InterpHandle {
    async fn eval(&mut self, f: impl EvalOnce) {
        self.co.yield_(FrameYield::Eval(f.into())).await;
    }
    async fn call(&mut self, s: Symbol) {
        self.co.yield_(FrameYield::Call(s)).await;
    }
    async fn stack_push(&mut self, v: impl Into<Val>) {
        self.co.yield_(FrameYield::StackPush(v.into())).await;
    }
    async fn pause(&mut self) {
        self.co.yield_(FrameYield::Pause).await;
    }
    async fn quote(&mut self) -> Option<Val> {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::Quote(Rc::clone(&r))).await;
        r.take()
    }
    async fn uplevel(&mut self, f: impl EvalOnce) {
        self.co.yield_(FrameYield::Uplevel(f.into())).await;
    }
}

// Rust code function

trait Eval: Into<ChildFrame> + Into<Definition> {}
trait EvalOnce: Into<ChildFrame> {}

type BuiltinFnRet = Pin<Box<dyn Future<Output = ()> + 'static>>;

#[derive(Clone)]
struct Builtin(Rc<dyn Fn(InterpHandle) -> BuiltinFnRet>);

impl std::fmt::Debug for Builtin {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl<F: 'static + Future<Output=()>,
     T: 'static + Eval + Fn(InterpHandle) -> F>
        From<T> for Builtin {
    fn from(f: T) -> Self {
        Builtin(Rc::new(move |i: InterpHandle| { Box::pin(f(i)) }))
    }
}
impl PartialEq for Builtin {
    fn eq(&self, that: &Self) -> bool { Rc::ptr_eq(&self.0, &that.0) }
}
impl Eq for Builtin {}
impl ImplValue for Builtin {}

impl<F: 'static + Future<Output=()>,
     T: 'static + Fn(InterpHandle) -> F>
     Eval for T {}

impl<F: 'static + Future<Output=()>,
     T: 'static + FnOnce(InterpHandle) -> F>
     EvalOnce for T {}

impl Eval for List<Val> {}
impl EvalOnce for List<Val> {}

// Code frame with a body being an in-progress Rust function
struct PausedFrame {
    body: Box<dyn Iterator<Item=FrameYield>>,
}
impl PausedFrame {
    fn next(&mut self) -> Option<FrameYield> {
        self.body.next()
    }
}

enum ChildFrame {
    ListFrame(ListFrame),
    PausedFrame(PausedFrame),
}
impl From<List<Val>> for ChildFrame {
    fn from(body: List<Val>) -> Self {
        ChildFrame::ListFrame(ListFrame { body, .. Default::default() })
    }
}

impl<F: 'static + Future<Output=()>,
     T: 'static + FnOnce(InterpHandle) -> F>
        From<T> for ChildFrame {
    fn from(f: T) -> Self {
        ChildFrame::PausedFrame(PausedFrame {
            body: Box::new(Gen::new(move |co| async move {
                f(InterpHandle { co }).await;
            }).into_iter()),
        })
    }
}
impl From<Builtin> for ChildFrame {
    fn from(b: Builtin) -> Self {
        ChildFrame::PausedFrame(PausedFrame {
            body: Box::new(Gen::new(move |co| async move {
                b.0(InterpHandle { co }).await;
            }).into_iter()),
        })
    }
}

#[derive(Debug, Clone)]
enum Definition {
    List(List<Val>),
    Builtin(Builtin),
}
impl From<List<Val>> for Definition {
    fn from(v: List<Val>) -> Self { Definition::List(v) }
}
impl<T: Into<Builtin>> From<T> for Definition {
    fn from(v: T) -> Self { Definition::Builtin(v.into()) }
}

impl Definition {
    fn start(self) -> ChildFrame {
        match self {
            Definition::List(body) => body.into(),
            Definition::Builtin(b) => b.into(),
        }
    }
}

#[derive(Default)]
pub struct Interpreter {
    frame: ListFrame,
    parents: Stack<ListFrame>,
    stack: Stack<Val>,
    defstacks: HashMap<String, Stack<Definition>>,
}

impl Interpreter {

    fn new(body: impl Into<List<Val>>) -> Self {
        Interpreter {
            frame: ListFrame { body: body.into(), ..ListFrame::default() },
            ..Interpreter::default()
        }
    }

    fn define(&mut self, name: impl Into<String>, def: impl Eval) {
        self.frame.defs.insert(name.into(), def.into());
    }

    // pub fn definition_get(&self, name: impl AsRef<str>) -> Option<&Definition> {
    // }

    fn definition_remove(&mut self, name: impl AsRef<String>) {
        self.frame.defs.remove(name.as_ref());
    }

    fn set_body(&mut self, body: List<Val>) { self.frame.body = body; }
    fn body_ref(&self) -> &List<Val> { &self.frame.body }

    fn is_toplevel(&self) -> bool { self.parents.empty() }

    // pub fn reset(&mut self) // in Interpreter only
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
    fn stack_pop_val(&mut self) -> Option<Val> { self.stack.pop() }
    fn stack_push(&mut self, v: impl Into<Val>) { self.stack.push(v.into()); }
    fn stack_len(&self) -> usize { self.stack.len() }

    fn resolve_ref(&self, s: &Symbol) -> Option<&Definition> {
        if let Some(def) = self.frame.defs.get(s.value()) {
            Some(def)
        } else if let Some(defstack) = self.defstacks.get(s.value()) {
            if let Some(def) = defstack.top() {
                Some(def)
            } else { None }
        } else { None }
    }

    fn read_body(&mut self) -> Option<Val> { self.frame.body.pop() }

    /// return complete
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

    // only used from InterpHandle
    // return whether to pause evaluation
    fn handle_yield(&mut self, y: FrameYield) -> bool {
        match y {
            FrameYield::Pause => return true,
            // FrameYield::Value(v) => { return Some(v); },
            FrameYield::ChildFrame(c) => self.frame.childs.push(c),
            FrameYield::Eval(v) => self.handle_eval(v),
            FrameYield::Call(v) => if !self.handle_call(v) { return true; },
            FrameYield::StackPush(v) => self.stack_push(v),
            FrameYield::Quote(yr) => self.handle_quote(yr),
            FrameYield::Uplevel(v) => if !self.handle_uplevel(v) { return true; },
        }
        false
    }

    fn create_call(&mut self, s: Symbol) {
        self.frame.childs.push((move |mut i: InterpHandle| async move {
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
            self.frame.childs.push(d.start());
            true
        } else {
            self.stack_push(List::from(vec![Symbol::new("undefined").into(), s.into()]));
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
                if self.defstacks.get(name).map_or(false, |x| x.empty()) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interp_basic() {
        // empty
        assert_eq!(Interpreter::new(vec![]).run(), true);
        // stack
        let mut i = Interpreter::new(vec![7.into()]);
        assert_eq!(i.stack_pop_val(), None);
        assert_eq!(i.run(), true);
        assert_eq!(i.stack_pop_val(), Some(7.into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    async fn toplevel_def(mut i: InterpHandle) {
        i.stack_push("yay").await;
    }

    #[test]
    fn interp_def() {
        let mut i =
            Interpreter::new(vec![
                Symbol::new("thingy").into(),
                Symbol::new("test").into(),
            ]);
        i.define("test", |mut i: InterpHandle| async move {
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
            Interpreter::new(vec![
                Symbol::new("quote").into(),
                Symbol::new("egg").into(),
            ]);
        i.define("quote", |mut i: InterpHandle| async move {
            if let Some(q) = i.quote().await {
                i.stack_push(q).await;
            }
        });
        assert_eq!(i.run(), true);
        assert_eq!(i.stack_pop_val(), Some(Symbol::new("egg").into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_uplevel() {
        let mut i =
            Interpreter::new(vec![
                Symbol::new("thing").into(),
                Symbol::new("egg").into(),
            ]);
        i.define("thing", List::from(vec![ Symbol::new("upquote").into() ]));
        i.define("upquote", |mut i: InterpHandle| async move {
            i.uplevel(|mut i: InterpHandle| async move {
                if let Some(q) = i.quote().await {
                    i.stack_push(q).await;
                }
            }).await;
        });
        assert_eq!(i.run(), true);
        assert_eq!(i.stack_pop_val(), Some(Symbol::new("egg").into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_uplevel_closure() {
        let mut i =
            Interpreter::new(vec![
                Symbol::new("thing").into(),
            ]);
        i.define("thing", List::from(vec![ Symbol::new("upfive").into() ]));
        i.define("upfive", |mut i: InterpHandle| async move {
            let five = Symbol::new("five");
            i.uplevel(move |mut i: InterpHandle| async move {
                i.stack_push(five).await;
            }).await;
        });
        assert_eq!(i.run(), true);
        assert_eq!(i.stack_pop_val(), Some(Symbol::new("five").into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_uplevel2() {
        let mut i =
            Interpreter::new(vec![
                Symbol::new("thing1").into(),
                Symbol::new("egg").into(),
            ]);
        i.define("thing1", List::from(vec![ Symbol::new("thing2").into() ]));
        i.define("thing2", List::from(vec![ Symbol::new("upquote2").into() ]));
        i.define("upquote2", |mut i: InterpHandle| async move {
            i.uplevel(move |mut i: InterpHandle| async move {
                i.uplevel(move |mut i: InterpHandle| async move {
                    if let Some(q) = i.quote().await {
                        i.stack_push(q).await;
                    }
                }).await;
            }).await;
        });
        assert_eq!(i.run(), true);
        assert_eq!(i.stack_pop_val(), Some(Symbol::new("egg").into()));
        assert_eq!(i.stack_pop_val(), None);
    }

    #[test]
    fn test_eval() {
        let mut i =
            Interpreter::new(vec![
                Symbol::new("eval").into(),
            ]);
        i.define("eval", |mut i: InterpHandle| async move {
            i.eval(List::from(vec![ Symbol::new("inner").into() ])).await;
        });
        i.define("inner", |mut i: InterpHandle| async move {
            i.eval(|mut i: InterpHandle| async move {
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
            Interpreter::new(vec![
                Symbol::new("five").into(),
            ]);
        i.define("five", |mut i: InterpHandle| async move {
            let five = Symbol::new("five");
            i.eval(move |mut i: InterpHandle| async move {
                i.stack_push(five).await;
            }).await;
        });
        assert_eq!(i.run(), true);
        assert_eq!(i.stack_pop_val(), Some(Symbol::new("five").into()));
        assert_eq!(i.stack_pop_val(), None);
    }

}

