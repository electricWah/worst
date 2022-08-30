
use std::cell::Cell;
use std::rc::Rc;
use core::pin::Pin;
use core::future::Future;
use std::collections::HashMap;
use genawaiter::{ rc::Gen, rc::Co };
use std::hash::{Hash, Hasher, BuildHasher};
use std::collections::hash_map::DefaultHasher;
use std::borrow::Borrow;

use crate::impl_value;
use crate::base::*;
use crate::list::List;


pub type YieldReturn<T> = Rc<Cell<Option<T>>>;

#[derive(Default)]
pub struct ListFrame {
    pub childs: Vec<ChildFrame>,
    pub body: List,
    pub meta: Meta,
    pub defs: DefSet,
}
impl ListFrame {
    pub fn new_body(body: List) -> Self {
        ListFrame { body, ..ListFrame::default() }
    }
    pub fn new_body_meta(body: List, meta: Meta) -> Self {
        ListFrame { body, meta, ..ListFrame::default() }
    }
    pub fn with_defs(mut self, defs: DefSet) -> Self {
        self.defs = defs;
        self
    }
    pub fn is_empty(&self) -> bool {
        self.childs.is_empty() && self.body.is_empty()
    }
}

#[derive(Clone)]
pub enum StackGetRequest {
    Any,
    OfType(Type),
}

impl StackGetRequest {
    pub fn of_type<T: ImplValue>() -> Self {
        Self::OfType(T::get_type())
    }
}

#[derive(Clone)]
pub struct StackGetOp {
    /// None = pop top, Some(nth) = get nth from top (0 being topmost)
    pub pop: Option<usize>,
    pub req: Vec<StackGetRequest>,
    pub res: YieldReturn<Vec<Val>>,
}
impl StackGetOp {
    pub fn from_request(pop: Option<usize>, v: Vec<StackGetRequest>) -> Self {
        Self {
            pop, req: v, res: Rc::new(Cell::new(None)),
        }
    }
    pub fn maybe_resolved(&mut self) -> Option<Vec<Val>> {
        self.res.take()
    }
    pub fn resolve_with(&mut self, res: Vec<Val>) {
        self.res.set(Some(res))
    }
}

pub enum FrameYield {
    Pause(Val),
    Eval(ChildFrame),
    Call(Symbol),
    Uplevel(ChildFrame),
    StackPush(Val),
    StackGetOp(StackGetOp),
    StackGetAll(YieldReturn<List>),
    Quote(YieldReturn<Val>),
    Define(String, Val),
    /// true: all definitions, false: only in current frame
    Definitions(bool, YieldReturn<DefSet>),
    ResolveDefinition(String, YieldReturn<Val>),
    GetCallStack(YieldReturn<Vec<Option<String>>>),
}

    pub struct PausedFrame {
        pub body: Box<dyn Iterator<Item=FrameYield>>,
    }

pub enum ChildFrame {
    ListFrame(ListFrame),
    PausedFrame(PausedFrame),
}
pub trait IntoChildFrame: Into<ChildFrame> {}
pub trait IntoVal { fn into_val(self) -> Val; }

/// Metadata for a definition (currently just name)
/// to make stack traces useful
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefineMeta {
    /// stack frame name
    pub name: String,
}
impl_value!(DefineMeta);

/// Runnable code. [List] and [Builtin] implement it.
pub trait Eval: IntoVal {}
/// Code that can run just once.
pub trait EvalOnce: IntoChildFrame {}

/// A reference to the currently-running [Interpreter] given to builtin functions.
pub struct Handle {
    pub(super) co: Co<FrameYield>,
}

/// A concrete [Eval] fn
#[derive(Clone)]
pub struct Builtin(Rc<dyn Fn(Handle) -> Pin<Box<dyn Future<Output = ()> + 'static>>>);
impl_value!(Builtin);

impl<F: 'static + Future<Output=()>,
     T: 'static + Eval + Fn(Handle) -> F>
        From<T> for Builtin {
    fn from(f: T) -> Self {
        Builtin(Rc::new(move |i: Handle| { Box::pin(f(i)) }))
    }
}

impl<F: 'static + Future<Output=()>,
     T: 'static + Fn(Handle) -> F>
     IntoVal for T {
    fn into_val(self) -> Val { Builtin::from(self).into() }
}
impl<F: 'static + Future<Output=()>,
     T: 'static + Fn(Handle) -> F>
     Eval for T {}

impl<F: 'static + Future<Output=()>,
     T: 'static + FnOnce(Handle) -> F>
     EvalOnce for T {}

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
impl<F: 'static + Future<Output=()>,
     T: 'static + FnOnce(Handle) -> F>
     IntoChildFrame for T {}

// TODO impl<T: Value> instead here?
impl Eval for Val {}
impl IntoVal for Val { fn into_val(self) -> Val { self } }
impl EvalOnce for Val {}
impl IntoChildFrame for Val {}
impl From<Val> for ChildFrame {
    fn from(v: Val) -> Self {
        let meta = v.meta_ref().clone();
        if v.is::<Builtin>() {
            v.downcast::<Builtin>().unwrap().into()
        } else if v.is::<List>() {
            child_frame_closure(v.downcast::<List>().unwrap(), &meta)
        } else {
            Builtin::from(move |mut i: Handle| {
                let vv = v.clone();
                async move {
                    i.stack_push(vv.clone()).await;
                }
            }).into()
        }
    }
}

fn child_frame_closure(l: List, meta: &Meta) -> ChildFrame {
    let mut frame = ListFrame::new_body_meta(l, meta.clone());
    if let Some(ClosureEnv(ds)) = meta.first_ref::<ClosureEnv>() {
        frame = frame.with_defs(ds.clone());
    }
    ChildFrame::ListFrame(frame)
}

impl Eval for Builtin {}
impl IntoVal for Builtin { fn into_val(self) -> Val { self.into() } }
impl EvalOnce for Builtin {}
impl IntoChildFrame for Builtin {}
impl From<Builtin> for ChildFrame {
    fn from(v: Builtin) -> Self {
        ChildFrame::PausedFrame(PausedFrame {
            body: Box::new(Gen::new(move |co| async move {
                v.0(Handle { co }).await;
            }).into_iter()),
        })
    }
}

/// Key for List meta to add env when evaling
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureEnv(pub(super) DefSet);
impl_value!(ClosureEnv);

// Code frame with a body being an in-progress Rust function
impl PausedFrame {
    pub fn next(&mut self) -> Option<FrameYield> {
        self.body.next()
    }
}

#[derive(Debug, Eq, Clone)]
struct PreHashed(String, u64);
impl Hash for PreHashed {
    fn hash<T: Hasher>(&self, h: &mut T) {
        h.write_u64(self.1);
    }
}
impl PartialEq for PreHashed {
    fn eq(&self, thee: &Self) -> bool {
        self.1 == thee.1 || self.0 == thee.0
    }
}

#[derive(Default)]
struct NoHasher(u64);
impl Hasher for NoHasher {
    fn finish(&self) -> u64 { let NoHasher(r) = self; *r }
    fn write(&mut self, data: &[u8]) { todo!("NoHasher write {:?}", data) }
    fn write_u64(&mut self, v: u64) { self.0 = v; }
}
#[derive(Clone, Default)]
struct BuildNoHasher;
impl BuildHasher for BuildNoHasher {
    type Hasher = NoHasher;
    fn build_hasher(&self) -> Self::Hasher { NoHasher::default() }
}

impl PreHashed {
    fn from_str(s: &str) -> Self {
        let mut h = DefaultHasher::new();
        s.hash(&mut h);
        PreHashed(s.to_string(), h.finish())
    }
    fn from_string(s: String) -> Self {
        let mut h = DefaultHasher::new();
        let st: &str = s.as_ref();
        st.hash(&mut h);
        PreHashed(s.to_string(), h.finish())
    }
}

/// Clone-on-write definition environment for list definitions.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DefSet(Rc<HashMap<PreHashed, Val, BuildNoHasher>>);
impl DefSet {
    /// Add a definition.
    pub fn insert(&mut self, key: String, val: impl Value) {
        Rc::make_mut(&mut self.0).insert(PreHashed::from_string(key), val.into());
    }
    /// Remove a definition by name.
    pub fn remove(&mut self, key: &str) {
        Rc::make_mut(&mut self.0).remove(&PreHashed::from_str(key));
    }
    /// Look for a definition by name.
    pub fn get(&self, key: impl AsRef<str>) -> Option<&Val> {
        self.0.get(&PreHashed::from_str(key.as_ref()))
    }
    fn keys_hashed(&self) -> impl Iterator<Item = &PreHashed> {
        self.0.keys()
    }
    /// An iterator over the contained definition names.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.keys().map(|k| &k.0)
    }
    fn iter_hashed(&self) -> impl Iterator<Item = (&PreHashed, &Val)> {
        self.0.iter()
    }
    /// An iterator over the contained definition name/body pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Val)> {
        self.0.iter().map(|(k, v)| (k.0.borrow(), v))
    }
    /// Whether there are no entries.
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    /// How many entries there are.
    pub fn len(&self) -> usize { self.0.len() }

    /// Retain definitions based on the given criterion.
    pub fn filter<F: Fn(&str, &Val) -> bool>(&mut self, f: F) {
        Rc::make_mut(&mut self.0).retain(|ph, v| f(ph.0.as_ref(), v));
    }
}

/// Stack of definitions for each def
#[derive(Default)]
pub struct DefStacks {
    data: HashMap<PreHashed, Vec<Val>, BuildNoHasher>,
}

impl DefStacks {
    pub fn iter_latest(&self) -> impl Iterator<Item=(&String, &Val)> {
        self.data.iter().filter_map(|(k, v)| v.last().map(|l| (&k.0, l)))
    }
    pub fn get_latest(&self, k: impl AsRef<str>) -> Option<&Val> {
        self.data.get(&PreHashed::from_str(k.as_ref())).and_then(|v| v.last())
    }
    pub fn push(&mut self, defs: &DefSet) {
        for (name, def) in defs.iter_hashed() {
            match self.data.get_mut(name) {
                Some(d) => d.push(def.clone()),
                None => {
                    self.data.insert(name.clone(), vec![def.clone()]);
                },
            }
        }
    }

    pub fn pop(&mut self, defs: &DefSet) {
        for name in defs.keys_hashed() {
            if self.data.get_mut(name)
                .map(|x| { x.pop(); x.is_empty() })
                    .unwrap_or(false) {
                        self.data.remove(name);
                    }
        }
    }
}

