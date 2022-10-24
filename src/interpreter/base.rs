
use std::cell::Cell;
use std::rc::Rc;
use core::pin::Pin;
use core::future::Future;
use std::collections::HashMap;
use genawaiter::{ rc::Gen, rc::Co };
use std::borrow::Borrow;

use crate::base::*;
use crate::list::List;

pub type YieldReturn<T> = Rc<Cell<Option<T>>>;

/// How to resolve definitions with resolve_definition
#[derive(Default)]
pub struct ResolveDefinition {
    /// Look in local definitions of stack frame
    locals: bool,
    /// Look in closure environment of stack frame
    defenv: bool,
    /// Continue looking up call stack if not found (dynamic)
    pub recursive_dynamic: bool,
    /// Look for definitions with this property (for e.g. avoiding infinite recursion)
    filter_fn: Option<Box<dyn Fn(&Val) -> bool>>,
}

pub enum FrameYield {
    Pause(Val),
    Eval(ToEvalOnce),
    /// EvalPre(pre, body)
    /// Evaluate `body`, but in the new child stack frame
    /// and before the first thing in `body`, eval `pre`.
    EvalPre(ToEvalOnce, List),
    Call(Symbol),
    Uplevel(ToEvalOnce),
    StackPush(Val),
    StackPop(YieldReturn<Val>),
    StackGetAll(YieldReturn<List>),
    Quote(YieldReturn<Val>),
    AddDefinition {
        name: String,
        def: Val,
    },
    Definitions {
        all: bool,
        ret: YieldReturn<DefSet>,
    },
    GetDefinition {
        name: String,
        resolver: ResolveDefinition,
        ret: YieldReturn<Val>,
    },
    RemoveDefinition {
        name: String,
        ret: YieldReturn<Val>,
    },
    GetCallStack(YieldReturn<Vec<Option<String>>>),
}

impl ResolveDefinition {
    pub fn local(mut self) -> Self { self.locals = true; self }
    pub fn environment(mut self) -> Self { self.defenv = true; self }
    pub fn dynamic(mut self) -> Self { self.recursive_dynamic = true; self }
    pub fn filter(mut self, f: impl Fn(&Val) -> bool + 'static) -> Self {
        self.filter_fn = Some(Box::new(f)); self
    }
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
    // Also dynamic definitions here? I think?
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
    pub fn add_definition(&mut self, name: impl Into<String>, def: impl Into<Val>) {
        self.locals.insert(name.into(), def.into());
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
    pub fn remove_local(&mut self, name: impl AsRef<str>) -> Option<Val> {
        self.locals.remove(name)
    }
    /// Find a definition according to the given resolver.
    /// Ignores the [recursive_dynamic] as that must be handled by the
    /// enclosing interpreter.
    pub fn resolve_definition(&self, name: impl AsRef<str>, resolver: &ResolveDefinition) -> Option<&Val> {
        if resolver.locals {
            if let Some(def) = self.locals.get(name.as_ref()) {
                match resolver.filter_fn.as_ref() {
                    Some(f) => { if f(&def) { return Some(def); } },
                    None => return Some(def),
                }
            }
        }
        if resolver.defenv {
            if let Some(def) = self.defenv.get(name.as_ref()) {
                match resolver.filter_fn.as_ref() {
                    Some(f) => { if f(&def) { return Some(def); } },
                    None => return Some(def),
                }
            }
        }
        None
    }
    pub fn find_def(&self, name: impl AsRef<str>) -> Option<&Val> {
        self.resolve_definition(name, &ResolveDefinition::default().local().environment())
    }
}

pub struct PausedFrame {
    pub body: Box<dyn Iterator<Item=FrameYield>>,
}

impl std::fmt::Debug for PausedFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<PausedFrame>")
    }
}

/// A reference to the currently-running [Interpreter] given to builtin functions.
pub struct Handle {
    pub(super) co: Co<FrameYield>,
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
        if self.is::<Builtin>() {
            self.downcast::<Builtin>().unwrap().into_eval_once()
        } else if self.is::<List>() {
            if let Some(defs) = self.meta_ref().first_ref::<DefSet>().cloned() {
                let meta = self.meta_ref().first_ref::<DefineMeta>().cloned().unwrap_or_default();
                ToEvalOnce::Def(self.downcast::<List>().unwrap(), meta, defs)
            } else {
                ToEvalOnce::Body(self.downcast::<List>().unwrap())
            }
        } else if self.is::<Symbol>() {
            self.downcast::<Symbol>().unwrap().into_eval_once()
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


/// A concrete [Eval] fn
#[derive(Clone)]
pub struct Builtin(Rc<dyn Fn(Handle) -> Pin<Box<dyn Future<Output = ()> + 'static>>>);
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
            body: Box::new(Gen::new(move |co| async move {
                self.0(Handle { co }).await;
            }).into_iter()),
        })
    }
}

impl<F: 'static + Future<Output=()>,
     T: 'static + FnOnce(Handle) -> F>
     EvalOnce for T {
    fn into_eval_once(self) -> ToEvalOnce {
        ToEvalOnce::Paused(PausedFrame {
            body: Box::new(Gen::new(move |co| async move {
                self(Handle { co }).await;
            }).into_iter()),
        })
    }
}

// Code frame with a body being an in-progress Rust function
impl PausedFrame {
    pub fn next(&mut self) -> Option<FrameYield> {
        self.body.next()
    }
}

/// Clone-on-write definition environment for list definitions.
#[derive(Default, Clone)]
pub struct DefSet(Rc<HashMap<String, Val>>);
impl Value for DefSet {}
impl DefSet {
    /// Add an evaluable definition.
    pub fn define(&mut self, key: String, val: impl Eval) {
        Rc::make_mut(&mut self.0).insert(key, val.into_val());
    }
    /// Add a regular definition.
    pub fn insert(&mut self, key: String, val: impl Into<Val>) {
        Rc::make_mut(&mut self.0).insert(key, val.into());
    }
    /// Remove a definition by name.
    pub fn remove(&mut self, key: impl AsRef<str>) -> Option<Val> {
        Rc::make_mut(&mut self.0).remove(key.as_ref())
    }
    /// Look for a definition by name.
    pub fn get(&self, key: impl AsRef<str>) -> Option<&Val> {
        self.0.get(key.as_ref())
    }
    /// An iterator over the contained definition names.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(|k| k.borrow())
    }
    /// An iterator over the contained definition name/body pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Val)> {
        self.0.iter().map(|(k, v)| (k.borrow(), v))
    }
    /// Whether there are no entries.
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    /// How many entries there are.
    pub fn len(&self) -> usize { self.0.len() }

    /// Retain definitions based on the given criterion.
    pub fn filter<F: Fn(&str, &Val) -> bool>(&mut self, f: F) {
        Rc::make_mut(&mut self.0).retain(|k, v| f(k.as_ref(), v));
    }

    /// Take everything from `thee` and put it in `self`.
    pub fn append(&mut self, thee: &DefSet) {
        if thee.is_empty() { return; }
        if self.is_empty() {
            *Rc::make_mut(&mut self.0) = (*thee.0).clone();
            return;
        }
        for (k, v) in thee.iter() {
            self.insert(k.into(), v.clone());
        }
    }
}

