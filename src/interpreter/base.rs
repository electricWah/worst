
use std::cell::Cell;
use std::rc::Rc;
use core::pin::Pin;
use core::future::Future;

use crate::base::*;
use crate::interpreter::async_gen::*;
use crate::interpreter::defset::*;

pub type YieldReturn<T> = Rc<Cell<Option<T>>>;

// pub enum ListYieldOp {
//     Pop(YieldReturn<Val>),
//     Get(YieldReturn<Val>),
//     Push(Val),
//     Set(List),
// }

// pub enum FrameYield {
//     Pause(Val),
//     Eval(ToEvalOnce),
//     Uplevel(ToEvalOnce), // could be empty and Multi(Vec<FrameYield>)
//     Call(Symbol), // could be Eval

//     Stack(ListYieldOp),
//     Body(ListYieldOp), // quote is Body(Pop)

//     DefinitionsGet(Option<DefSetType>, YieldReturn<DefSet>),
//     DefinitionsSet(DefSetType, DefSet),
//     // GetCallStack(YieldReturn<Vec<Option<String>>>),
// }

/// A scope to specify where to get or add definitions.
#[derive(PartialEq, Eq)]
pub enum DefScope {
    /// Local definitions start out empty.
    Local,
    /// DefEnv definitions are inherited from the defining environment.
    DefEnv,
}

pub enum FrameYield {
    Pause(Val),
    Eval(ToEvalOnce),
    Call(Symbol),
    Uplevel(ToEvalOnce),
    IsToplevel(YieldReturn<bool>),
    StackPush(Val),
    StackPop(YieldReturn<Val>),
    StackGetAll(YieldReturn<List>),
    Quote(YieldReturn<Val>),
    GetDefinitions {
        scope: Option<DefScope>, // None = All
        ret: YieldReturn<DefSet>,
    },
    SetDefinitions {
        scope: DefScope,
        defs: DefSet,
    },
    AddDefinition {
        name: String,
        def: Val,
        scope: DefScope,
    },
    GetDefinition {
        name: String,
        scope: Option<DefScope>, // None = local then defenv
        ret: YieldReturn<Val>,
    },
    RemoveDefinition {
        name: String,
        ret: YieldReturn<Val>,
    },
    GetCallStack(YieldReturn<Vec<Option<String>>>),
}

/// Metadata for a definition (currently just name)
/// to make stack traces useful
// (should ClosureEnv go in here?)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DefineMeta {
    /// Name of definition
    pub name: Option<String>,
}
impl Value for DefineMeta {}

#[must_use]
pub enum ChildFrame {
    ListFrame(ListFrame),
    PausedFrame(PausedFrame),
}

#[derive(Default)]
pub struct ListFrame {
    pub childs: Vec<ChildFrame>,
    pub body: List,
    pub meta: DefineMeta,
    pub defenv: DefSet,
    pub locals: DefSet,
}

impl ListFrame {
    pub fn is_empty(&self) -> bool {
        self.childs.is_empty() && self.body.is_empty()
    }
    pub fn def_name(&self) -> Option<&str> {
        self.meta.name.as_ref().map(|s| s.as_ref())
    }
    pub fn all_defs(&self) -> DefSet {
        let mut defs = self.defenv.clone();
        defs.append(&self.locals);
        defs
    }

    pub fn eval_list(&self, body: List) -> ListFrame {
        ListFrame { body, defenv: self.all_defs(), ..ListFrame::default() }
    }
    pub fn eval_once(&mut self, eval: ToEvalOnce) -> ChildFrame {
        match eval {
            ToEvalOnce::Def(body, meta, defenv) =>
                ChildFrame::ListFrame(ListFrame {
                    body, meta, defenv, ..ListFrame::default()
                }),
            ToEvalOnce::Body(body) =>
                ChildFrame::ListFrame(self.eval_list(body)),
            ToEvalOnce::Paused(p) =>
                ChildFrame::PausedFrame(p),
        }
    }

    pub fn get_definition(&self, name: impl AsRef<str>, scope: Option<DefScope>) -> Option<&Val> {
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
}

pub struct PausedFrame {
    pub body: Generator<FrameYield>,
}

impl PausedFrame {
    pub fn next(&mut self) -> Option<FrameYield> {
        self.body.next()
    }
}

impl std::fmt::Debug for PausedFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<PausedFrame>")
    }
}

/// A reference to the currently-running [Interpreter] given to builtin functions.
pub struct Handle {
    inner: Ctx<FrameYield>,
}

impl Handle {
    fn new(inner: Ctx<FrameYield>) -> Self {
        Handle { inner }
    }
    pub(super) async fn yield_(&self, v: FrameYield) {
        self.inner.yield_(v).await
    }
}

// TODO simplify all of this stuff

/// Runnable code. [List] and [Builtin] implement it.
pub trait Eval {
    fn into_val(self) -> Val;
    /// Whether this requires eval_meta to be added (DefineMeta, DefSet)
    /// (should be true for lists, false for builtins)
    fn eval_meta(&self) -> bool;
}

#[must_use]
pub enum ToEvalOnce {
    Def(List, DefineMeta, DefSet),
    Body(List),
    Paused(PausedFrame),
}

/// Code that can run once.
pub trait EvalOnce {
    fn into_eval_once(self) -> ToEvalOnce;
    // fn eval_once(&self, frame: &mut ListFrame);
}

impl Eval for Val {
    fn into_val(self) -> Val { self }
    fn eval_meta(&self) -> bool { self.is::<List>() }
}

impl EvalOnce for Val {
    fn into_eval_once(self) -> ToEvalOnce {
        // TODO match try_downcast
        if self.is::<Builtin>() {
            self.try_downcast::<Builtin>().ok().unwrap().into_inner().into_eval_once()
        } else if self.is::<List>() {
            if let Some(defs) = self.meta_ref().first_ref::<DefSet>().cloned() {
                let meta = self.meta_ref().first_ref::<DefineMeta>().cloned().unwrap_or_default();
                ToEvalOnce::Def(self.try_downcast::<List>().ok().unwrap().into_inner(), meta, defs)
            } else {
                ToEvalOnce::Body(self.try_downcast::<List>().ok().unwrap().into_inner())
            }
        } else if self.is::<Symbol>() {
            self.try_downcast::<Symbol>().ok().unwrap().into_inner().into_eval_once()
        } else {
            (move |mut i: Handle| {
                let vv = self.clone();
                async move {
                    i.stack_push(vv.clone()).await;
                }
            }).into_eval_once()
        }
    }
}

impl EvalOnce for List {
    fn into_eval_once(self) -> ToEvalOnce {
        ToEvalOnce::Body(self)
    }
}

impl EvalOnce for Symbol {
    fn into_eval_once(self) -> ToEvalOnce {
        (move |mut i: Handle| {
            async move {
                i.call(self.clone()).await;
            }
        }).into_eval_once()
    }
}

type BuiltinRet<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

/// A concrete [Eval] fn
#[derive(Clone)]
pub struct Builtin(Rc<dyn Fn(Handle) -> BuiltinRet<()>>);
impl Value for Builtin {}

impl<F: 'static + Future<Output=()>,
     T: 'static + Fn(Handle) -> F>
        From<T> for Builtin {
    fn from(f: T) -> Self {
        Builtin(Rc::new(move |i: Handle| { Box::pin(f(i)) }))
    }
}

impl<T: Into<Builtin>> Eval for T {
    fn into_val(self) -> Val { Val::from(self.into()) }
    fn eval_meta(&self) -> bool { false }
}

impl EvalOnce for Builtin {
    fn into_eval_once(self) -> ToEvalOnce {
        ToEvalOnce::Paused(PausedFrame {
            body: Generator::new(|x| async move {
                self.0(Handle::new(x)).await;
            }),
        })
    }
}

impl<F: 'static + Future<Output=()>,
     T: 'static + FnOnce(Handle) -> F>
     EvalOnce for T {
    fn into_eval_once(self) -> ToEvalOnce {
        ToEvalOnce::Paused(PausedFrame {
            body: Generator::new(|x| async move {
                self(Handle::new(x)).await;
            }),
        })
    }
}

