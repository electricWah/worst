
use std::fmt::Debug;
use downcast_rs::Downcast;

pub trait Value: Downcast + Debug {
    fn dup(&self) -> Val;
    fn eq(&self, other: &Val) -> bool;
    // fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>;
}
downcast_rs::impl_downcast!(Value);

pub type Val = Box<dyn Value>;

impl Clone for Val {
    fn clone(&self) -> Val { self.dup() }
}
impl PartialEq for Val {
    fn eq(&self, that: &Self) -> bool { Value::eq(self.as_ref(), that) }
}
impl Eq for Val { }

pub trait ImplValue: Clone + Eq {}
impl<T: 'static> Value for T where T: ImplValue + Debug {
    fn dup(&self) -> Val {
        Box::new(self.clone())
    }
    fn eq(&self, that: &Val) -> bool {
        if let Some(t) = that.downcast_ref::<T>() { self == t } else { false }
    }
}

impl<T: ImplValue + Debug + 'static> From<T> for Val {
    fn from(t: T) -> Self { Box::new(t) }
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

