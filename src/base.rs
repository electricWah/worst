
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
    pub fn new<T: ToString>(t: T) -> Self { Symbol { v: t.to_string() } }
    pub fn value(&self) -> &String { &self.v }
}

impl ImplValue for Symbol {}
impl ImplValue for String {}
impl ImplValue for bool {}
impl ImplValue for i32 {} // TODO any numeric

impl From<&'static str> for Val {
    fn from(s: &'static str) -> Val { String::from(s).into() }
}

#[derive(Clone, Debug)]
pub struct Stack<T> {
    data: Vec<T>,
}
impl<T> Default for Stack<T> {
    fn default() -> Self { Stack { data: vec![] } }
}

impl<T> Stack<T> {
    pub fn len(&self) -> usize { self.data.len() }
    pub fn empty(&self) -> bool { self.len() == 0 }
    pub fn push(&mut self, v: T) { self.data.push(v) }
    pub fn pop(&mut self) -> Option<T> { self.data.pop() }
    pub fn top(&self) -> Option<&T> {
        if self.empty() { None } else { Some(&self.data[self.data.len() - 1]) }
    }
}

