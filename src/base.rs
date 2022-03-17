
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use downcast_rs::Downcast;

pub trait Value: Downcast + Debug {
    fn to_val(self) -> Val; // saves dorking around with meta in pub Val::new
    fn dup(&self) -> Val;
    fn equal(&self, other: &Val) -> bool;
}
downcast_rs::impl_downcast!(Value);

#[derive(Debug)]
pub struct Val {
    v: Box<dyn Value>,
    pub meta: Vec<Val>,
    // meta..?
}

impl Clone for Val {
    fn clone(&self) -> Val {
        Val {
            v: self.v.dup().v,
            meta: self.meta.clone(),
        }
    }
}
impl PartialEq for Val {
    fn eq(&self, that: &Self) -> bool { Value::equal(self.v.as_ref(), that) }
}
impl Eq for Val { }

impl Value for Val {
    fn to_val(self) -> Val { self }
    fn dup(&self) -> Val { self.clone() }
    fn equal(&self, other: &Val) -> bool { self.v.equal(other) }
}

impl Val {
    fn new(v: impl Value) -> Self { Val { v: Box::new(v), meta: vec![] } }
    pub fn downcast<T: Value>(self) -> Result<T, Val> {
        match self.v.downcast::<T>() {
            Ok(v) => Ok(*v),
            Err(v) => Err(Val { v, meta: self.meta }),
        }
    }
    pub fn downcast_ref<T: Value>(&self) -> Option<&T> {
        self.v.downcast_ref::<T>()
    }
    pub fn is<T: Value>(&self) -> bool {
        self.v.is::<T>()
    }
    pub fn deconstruct(self) -> (Box<dyn Value>, Vec<Val>) { (self.v, self.meta) }
    pub fn add_meta(&mut self, v: impl Value) { self.meta.push(Val::new(v)); }
    pub fn with_meta(mut self, v: impl Value) -> Self { self.add_meta(v); self }
}

pub trait ImplValue: Clone + Eq {}
impl<T: 'static> Value for T where T: ImplValue + Debug {
    fn to_val(self) -> Val { Val::new(self) }
    fn dup(&self) -> Val { Val::new(self.clone()) }
    fn equal(&self, that: &Val) -> bool {
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

