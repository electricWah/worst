
use std::cell::RefCell;
use std::fmt::{ Debug, Display };
use std::rc::Rc;

use super::value::*;

/// Symbol type: an unquoted word used to look up definitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    v: String,
}
impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.v)
    }
}
impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str { self.v.as_ref() }
}
/// Conversion into a symbol.
///
/// May be removed in favour of [Symbol::from].
pub trait ToSymbol {
    /// Convert this into a [Symbol].
    fn to_symbol(self) -> Symbol;
}
impl<T: Into<Symbol>> ToSymbol for T {
    fn to_symbol(self) -> Symbol { self.into() }
}

impl From<&str> for Symbol {
    fn from(s: &str) -> Symbol { Symbol { v: s.to_string() } }
}
impl From<String> for Symbol {
    fn from(v: String) -> Symbol { Symbol { v } }
}
impl From<Symbol> for String {
    fn from(s: Symbol) -> Self { s.v }
}

impl Value for Symbol {}
impl Value for bool {}
impl Value for String {}
impl Value for i64 {}
impl Value for f64 {}

/// Mutable memory location (a wrapper for [RefCell]).
#[derive(Clone)]
pub struct Place(Rc<RefCell<Val>>);
impl Value for Place {}

impl Place {
    /// Create a new [Place] wrapping `v`.
    pub fn wrap(v: impl Into<Val>) -> Place {
        Place(Rc::new(RefCell::new(v.into())))
    }
    /// Trade the contents of this [Place] with a new value.
    pub fn swap(&mut self, v: impl Into<Val>) -> Val {
        self.0.replace(v.into())
    }
    /// Update the contents of this [Place], discarding the old value.
    pub fn set(&mut self, v: impl Into<Val>) { self.swap(v); }

    /// Get a copy of the contained [Val].
    pub fn get(&self) -> Val { self.0.try_borrow().unwrap().clone() }
}

/// Meta value signalling that the value represents some kind of error.
///
/// Set IsError on all members of a type:
/// ```ignore
/// struct BadSituation;
/// impl_value!(BadSituation, IsError);
/// assert!(IsError::is_error(&BadSituation));
/// ```
/// Set IsError on a single value:
/// ```ignore
/// let mut v = IsError::add("an error".to_string());
/// assert!(IsError::is_error(&v));
/// ```
pub struct IsError;
impl Value for IsError {}
impl IsError {
    /// Add IsError metadata to the value.
    pub fn add(v: impl Into<Val>) -> Val {
        let mut v: Val = v.into();
        v.meta_ref_mut().push(IsError);
        v
    }
    /// Check whether the value or its type is an error.
    pub fn is_error(v: &Val) -> bool {
        v.meta_ref().contains::<Self>()
    }
}

