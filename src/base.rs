
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

#[derive(Default, Debug, Clone)]
pub struct Meta(Rc<Vec<Val>>);

#[derive(Debug, Clone)]
pub struct Val {
    v: Rc<Box<dyn Value>>,
    pub meta: Meta,
}

impl PartialEq for Val {
    fn eq(&self, that: &Self) -> bool { Value::equal(self.v.as_ref().as_ref(), that) }
}
impl Eq for Val { }

impl Value for Val {
    fn to_val(self) -> Val { self }
    fn dup(&self) -> Val { self.clone() }
    fn equal(&self, other: &Val) -> bool { self.v.equal(other) }
}

impl Val {
    fn new(v: impl Value) -> Self {
        Val { v: Rc::new(Box::new(v)), meta: Meta::default() }
    }
    pub fn downcast<T: Value>(self) -> Result<T, Val> {
        match Rc::try_unwrap(self.v) {
            Ok(v) => {
                match v.downcast::<T>() {
                    Ok(v) => Ok(*v),
                    Err(v) => Err(Val { v: Rc::new(v), meta: self.meta }),
                }
            },
            Err(e) => { e.dup().downcast::<T>() },
        }
    }
    pub fn downcast_ref<T: Value>(&self) -> Option<&T> {
        self.v.downcast_ref::<T>()
    }
    pub fn downcast_mut<T: Value>(&mut self) -> Option<&mut T> {
        if let Some(v) = Rc::get_mut(&mut self.v) {
            v.downcast_mut::<T>()
        } else {
            None
            //Rc::make_mut(&mut self.v).downcast_mut::<T>()
        }
    }
    pub fn is<T: Value>(&self) -> bool {
        self.v.is::<T>()
    }
    pub fn add_meta(&mut self, v: impl Value) { self.meta.push(Val::new(v)); }
    pub fn with_meta(mut self, v: impl Value) -> Self { self.add_meta(v); self }
    pub fn get_meta(&self) -> Meta { self.meta.clone() }
}

impl Meta {
    fn push(&mut self, v: impl Into<Val>) {
        Rc::make_mut(&mut self.0).push(v.into());
    }
    pub fn first<T: Value>(&self) -> Option<&T> {
        self.0.iter().find_map(|v| v.downcast_ref::<T>())
    }
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
impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str { self.v.as_ref() }
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

impl Value for &'static str {
    fn to_val(self) -> Val { self.to_string().to_val() }
    fn dup(&self) -> Val { self.to_string().dup() }
    fn equal(&self, other: &Val) -> bool { self.to_string().equal(other) }
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

