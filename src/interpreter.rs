
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

type YieldReturn<T> = Rc<Cell<Option<T>>>;

// Don't want to leak ChildFrame required by Eval/EvalOnce
mod private {
    use super::*;

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
    }

    pub enum FrameYield {
        Pause,
        Eval(ChildFrame),
        Call(Symbol),
        StackPush(Val),
        StackPop(YieldReturn<Val>),
        StackGet(YieldReturn<List>),
        Quote(YieldReturn<Val>),
        Uplevel(ChildFrame),
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
}
use private::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct DefineMeta {
    name: String,
}
impl_value!(DefineMeta);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpError {
    StackEmpty(Vec<Option<String>>),
    WrongType(Val, &'static str, Vec<Option<String>>),
}
impl_value!(InterpError);

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
    pub async fn eval_child(&mut self, body: List, child: impl EvalOnce) {
        let mut frame = ListFrame::new_body(body);
        frame.childs.push(child.into());
        self.co.yield_(FrameYield::Eval(ChildFrame::ListFrame(frame))).await;
    }
    pub async fn call(&mut self, s: impl Into<Symbol>) {
        self.co.yield_(FrameYield::Call(s.into())).await;
    }
    pub async fn stack_push(&mut self, v: impl Value) {
        self.co.yield_(FrameYield::StackPush(v.into())).await;
    }

    pub async fn stack_pop_val(&mut self) -> Val {
        let r = Rc::new(Cell::new(None));
        loop {
            self.co.yield_(FrameYield::StackPop(Rc::clone(&r))).await;
            match r.take() {
                Some(v) => return v,
                None => {
                    let cs = self.call_stack_names().await;
                    self.stack_push(InterpError::StackEmpty(cs)).await;
                }
            }
        }
    }
    // TODO stack_pop_meta
    pub async fn stack_pop<T: Value + Clone>(&mut self) -> T {
        loop {
            let v = self.stack_pop_val().await;
            match v.downcast::<T>() {
                Ok(r) => return r,
                Err(e) => {
                    self.stack_push(e.clone()).await;
                    let name = core::any::type_name::<T>();
                    let cs = self.call_stack_names().await;
                    self.stack_push(dbg!(InterpError::WrongType(e, name, cs))).await;
                    self.pause().await;
                },
            }
        }
    }
    pub async fn stack_get(&mut self) -> List {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::StackGet(Rc::clone(&r))).await;
        r.take().unwrap()
    }
    pub async fn stack_empty(&mut self) -> bool {
        self.stack_get().await.len() == 0
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
        self.co.yield_(FrameYield::Define(name.into(), def.into_val())).await;
    }
    pub async fn define_closure(&mut self, name: impl Into<String>,
                                body: impl Value, env: DefSet) {
        let v = body.into().with_meta(ClosureEnv(env));
        self.co.yield_(FrameYield::Define(name.into(), v)).await;
    }
    async fn get_definitions(&mut self, global: bool) -> DefSet {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::Definitions(global, Rc::clone(&r))).await;
        r.take().unwrap()
    }
    pub async fn local_definitions(&mut self) -> DefSet {
        self.get_definitions(false).await
    }
    pub async fn all_definitions(&mut self) -> DefSet {
        self.get_definitions(true).await
    }
    pub async fn resolve_definition(&mut self, name: impl Into<String>) -> Option<Val> {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::ResolveDefinition(name.into(), Rc::clone(&r))).await;
        r.take()
    }
    pub async fn call_stack_names(&mut self) -> Vec<Option<String>> {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::GetCallStack(Rc::clone(&r))).await;
        r.take().unwrap()
    }
}

pub trait Eval: IntoVal {}
pub trait EvalOnce: IntoChildFrame {}

// Concrete type for an Eval fn
#[derive(Clone)]
pub struct Builtin(Rc<dyn Fn(Handle) -> Pin<Box<dyn Future<Output = ()> + 'static>>>);

impl std::fmt::Debug for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Builtin>")?;
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
impl_value!(Builtin);

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
            todo!("eval {:?}", v)
        }
    }
}

fn child_frame_closure(l: List, meta: &Meta) -> ChildFrame {
    let mut frame = ListFrame::new_body_meta(l, meta.clone());
    if let Some(ClosureEnv(ds)) = meta.first::<ClosureEnv>() {
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
struct ClosureEnv(DefSet);
impl_value!(ClosureEnv);

// Code frame with a body being an in-progress Rust function
impl PausedFrame {
    fn next(&mut self) -> Option<FrameYield> {
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
    fn finish(&self) -> u64 { let NoHasher(r) = self; r.clone() }
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

/// Clone-on-write definition environment for list definitions
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DefSet(Rc<HashMap<PreHashed, Val, BuildNoHasher>>);
impl DefSet {
    pub fn insert(&mut self, key: String, val: impl Value) {
        Rc::make_mut(&mut self.0).insert(PreHashed::from_string(key), val.into());
    }
    pub fn remove(&mut self, key: &str) {
        Rc::make_mut(&mut self.0).remove(&PreHashed::from_str(key));
    }
    pub fn get(&self, key: impl AsRef<str>) -> Option<&Val> {
        self.0.get(&PreHashed::from_str(key.as_ref()))
    }
    fn keys_hashed(&self) -> impl Iterator<Item = &PreHashed> {
        self.0.keys()
    }
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.keys().map(|k| &k.0)
    }
    fn iter_hashed(&self) -> impl Iterator<Item = (&PreHashed, &Val)> {
        self.0.iter()
    }
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Val)> {
        self.0.iter().map(|(k, v)| (k.0.borrow(), v))
    }
    pub fn len(&self) -> usize { self.0.len() }

    pub fn filter<F: Fn(&str, &Val) -> bool>(&mut self, f: F) {
        Rc::make_mut(&mut self.0).retain(|ph, v| f(ph.0.as_ref(), &v));
    }
}

/// Stack of definitions for each def
#[derive(Default)]
struct DefStacks {
    data: HashMap<PreHashed, Vec<Val>, BuildNoHasher>,
}

impl DefStacks {
    fn iter_latest(&self) -> impl Iterator<Item=(&String, &Val)> {
        self.data.iter().filter_map(|(k, v)| match v.last() {
            Some(l) => Some((&k.0, l)), None => None
        })
    }
    fn get_latest(&self, k: impl AsRef<str>) -> Option<&Val> {
        self.data.get(&PreHashed::from_str(k.as_ref())).map(|v| v.last()).flatten()
    }
    fn push(&mut self, defs: &DefSet) {
        for (name, def) in defs.iter_hashed() {
            match self.data.get_mut(&name) {
                Some(d) => d.push(def.clone()),
                None => {
                    self.data.insert(name.clone(), vec![def.clone()]);
                },
            }
        }
    }

    fn pop(&mut self, defs: &DefSet) {
        for name in defs.keys_hashed() {
            if self.data.get_mut(name)
                .map(|x| { x.pop(); x.is_empty() })
                    .unwrap_or(false) {
                        self.data.remove(name);
                    }
        }
    }
}

#[derive(Default)]
pub struct Paused {
    frame: ListFrame,
    parents: Vec<ListFrame>,
    stack: List,
    defstacks: DefStacks,
}

impl std::fmt::Debug for Paused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<paused interpreter>")
    }
}

impl Paused {

    // pub fn eval(&mut self, f: impl EvalOnce) {
    //     self.handle_eval(f.into());
    //     while !self.run() {}
    // }

    fn new(body: impl Into<List>) -> Self {
        Paused {
            frame: ListFrame::new_body(body.into()),
            ..Paused::default()
        }
    }

    pub fn define(&mut self, name: impl Into<String>, def: impl Eval) {
        let name = name.into();
        let defmeta = DefineMeta { name: name.clone() };
        self.frame.defs.insert(name, def.into_val().with_meta(defmeta));
    }

    // pub fn definition_get(&self, name: impl AsRef<str>) -> Option<&Definition> {
    // }

    pub fn definition_remove(&mut self, name: impl AsRef<str>) {
        self.frame.defs.remove(name.as_ref());
    }

    pub fn all_definitions(&self) -> DefSet {
        let mut defs = self.frame.defs.clone();
        for (name, def) in self.defstacks.iter_latest() {
            defs.insert(name.clone(), def.clone());
        }
        defs
    }

    // pub fn set_body(&mut self, body: List<Val>) { self.frame.body = body; }
    // fn body_ref(&self) -> &List<Val> { &self.frame.body }

    pub fn is_toplevel(&self) -> bool { self.parents.len() == 0 }

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

    // also stack_pop_purpose() etc etc
    // maybe stack() + stack_mut() -> StackRef
    // with pop_any() and pop::<T> and ref etc?

    pub fn stack_ref(&self) -> &List { &self.stack }
    pub fn stack_pop_val(&mut self) -> Option<Val> { self.stack.pop() }
    pub fn stack_push(&mut self, v: impl Into<Val>) { self.stack.push(v.into()); }
    pub fn stack_len(&self) -> usize { self.stack.len() }

    fn resolve_ref(&self, s: impl AsRef<str>) -> Option<&Val> {
        if let Some(def) = self.frame.defs.get(s.as_ref()) {
            Some(def)
        } else if let Some(def) = self.defstacks.get_latest(s) {
            Some(def)
        } else { None }
    }

    fn read_body(&mut self) -> Option<Val> { self.frame.body.pop() }

    /// returns complete
    pub fn run(&mut self) -> bool {
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
                            self.call(s.clone());
                        } else {
                            self.stack_push(next);
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
            FrameYield::Define(name, def) => self.define(name, def),
            FrameYield::Definitions(false, yr) => yr.set(Some(self.frame.defs.clone())),
            FrameYield::Definitions(true, yr) => yr.set(Some(self.all_definitions())),
            FrameYield::ResolveDefinition(name, yr) => yr.set(self.resolve_ref(&name).map(Val::clone)),
            FrameYield::GetCallStack(yr) => yr.set(Some(self.call_stack_names())),
        }
        false
    }

    /// Immediately call `name` when the interpreter is next resumed
    pub fn call(&mut self, name: impl Into<Symbol>) {
        let s = name.into();
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
        if let Some(DefineMeta { name }) = self.frame.meta.first::<DefineMeta>() {
            r.push(Some(name.clone()));
        } else {
            r.push(None);
        }

        for p in self.parents.iter().rev() {
            if let Some(DefineMeta { name }) = p.meta.first::<DefineMeta>() {
                r.push(Some(name.clone()));
            } else {
                r.push(None);
            }
        }

        r
    }

}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Builder {
    stack: List,
    defs: HashMap<String, Val>,
}

impl Builder {
    fn install(&self, i: &mut Paused) {
        i.stack = self.stack.clone();
        for (k, v) in self.defs.iter() { i.define(k, v.clone()); }
    }
    pub fn eval(&self, f: impl EvalOnce) -> Paused {
        match f.into() {
            ChildFrame::ListFrame(frame) => {
                let mut i = Paused { frame, ..Paused::default() };
                self.install(&mut i);
                i
            },
            paused@ChildFrame::PausedFrame(_) => {
                let mut i = Paused::new(vec![]);
                self.install(&mut i);
                i.frame.childs.push(paused);
                i
            },
        }
    }

    pub fn with_stack(mut self, s: impl Into<List>) -> Self {
        self.stack = s.into();
        self
    }
    pub fn define(&mut self, name: impl Into<String>, def: impl Eval) {
        let name = name.into();
        let defmeta = DefineMeta { name: name.clone() };
        self.defs.insert(name, def.into_val().with_meta(defmeta));
    }
    pub fn with_define(mut self, name: impl Into<String>, def: impl Eval) -> Self {
        self.define(name, def);
        self
    }
    pub fn define_closure(mut self, name: impl Into<String>,
                          body: List, env: DefSet) -> Self {
        self.defs.insert(name.into(), Val::from(body).with_meta(ClosureEnv(env)));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interp_basic() {
        // empty
        assert!(Builder::default().eval(List::from(vec![]).into()).run());
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
        i.define("thing", List::from(vec![ "upquote".to_symbol().into() ]).into());
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
        i.define("thing", List::from(vec![ "upfive".to_symbol().into() ]).into());
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
        i.define("thing1", List::from(vec![ "thing2".to_symbol().into() ]).into());
        i.define("thing2", List::from(vec![ "upquote2".to_symbol().into() ]).into());
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
            i.eval(List::from(vec![ "inner".to_symbol().into() ]).into()).await;
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

