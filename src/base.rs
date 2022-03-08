
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use downcast_rs::Downcast;

pub trait Value: Downcast + Debug {
    fn dup(&self) -> Val;
    fn eq(&self, other: &Val) -> bool;
    // fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>;
}
downcast_rs::impl_downcast!(Value);

#[derive(Debug)]
pub struct Val {
    v: Box<dyn Value>,
    // meta..?
}

impl Clone for Val {
    fn clone(&self) -> Val { self.dup() }
}
impl PartialEq for Val {
    fn eq(&self, that: &Self) -> bool { Value::eq(self.v.as_ref(), that) }
}
impl Eq for Val { }

impl Value for Val {
    fn dup(&self) -> Val { self.v.dup() }
    fn eq(&self, other: &Val) -> bool { self.v.eq(other) }
}

impl Val {
    fn new<T: Value>(v: T) -> Self { Val { v: Box::new(v) } }
    pub fn downcast<T: Value>(self) -> Result<T, Val> {
        match self.v.downcast::<T>() {
            Ok(v) => Ok(*v),
            Err(v) => Err(Val { v }),
        }
    }
    pub fn downcast_ref<T: Value>(&self) -> Option<&T> {
        self.v.downcast_ref::<T>()
    }
    pub fn is<T: Value>(&self) -> bool {
        self.v.is::<T>()
    }
}

pub trait ImplValue: Clone + Eq {}
impl<T: 'static> Value for T where T: ImplValue + Debug {
    fn dup(&self) -> Val {
        Val::new(self.clone())
    }
    fn eq(&self, that: &Val) -> bool {
        if let Some(t) = that.downcast_ref::<T>() { self == t } else { false }
    }
}

impl<T: ImplValue + Debug + 'static> From<T> for Val {
    fn from(t: T) -> Self { Val::new(t) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    v: String,
}
impl Symbol {
    pub fn as_string(&self) -> &String { &self.v }
}
pub trait ToSymbol { fn to_symbol(self) -> Symbol; }
impl<T: Into<Symbol>> ToSymbol for T {
    fn to_symbol(self) -> Symbol { self.into() }
}

impl From<&str> for Symbol {
    fn from(s: &str) -> Symbol { Symbol { v: s.to_string() } }
}
impl From<Symbol> for String {
    fn from(s: Symbol) -> Self { s.v }
}

impl ImplValue for Symbol {}
impl ImplValue for String {}
impl ImplValue for bool {}
impl ImplValue for i32 {} // TODO any numeric

impl From<&'static str> for Val {
    fn from(s: &'static str) -> Val { String::from(s).into() }
}

#[derive(Debug, Clone, Eq)]
pub struct Place(Rc<RefCell<Val>>);
impl PartialEq for Place {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl ImplValue for Place {}

impl Place {
    pub fn wrap(v: impl Value) -> Place {
        Place(Rc::new(RefCell::new(Val::new(v))))
    }
    pub fn swap(&mut self, v: impl Value) -> Val {
        self.0.replace(Val::new(v))
    }
    pub fn set(&mut self, v: impl Value) { self.swap(v); }

    pub fn get(&self) -> Val { self.0.try_borrow().unwrap().clone() }
}

