
use std::cell::RefCell;
use std::fmt::{ Debug, Display };
use std::rc::Rc;

use crate::impl_value;
use super::value::*;
use super::traits::*;

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

impl_value!(Symbol, value_tostring(Symbol::to_string), type_name("symbol"));
fn bool_tostring(b: &bool) -> String { (if *b { "#t" } else { "#f" }).into() }
impl_value!(bool, value_tostring(bool_tostring));
impl_value!(String, value_debug::<String>(), type_name("string"));

// TODO bunch of numbers, better than this
impl_value!(i64, value_debug::<i64>(), type_name("int64"));
impl_value!(f64, value_debug::<f64>(), type_name("float64"));

/// Mutable memory location (a wrapper for [RefCell]).
#[derive(Clone)]
pub struct Place(Rc<RefCell<Val>>);
impl_value!(Place, type_name("place"));

impl Place {
    /// Create a new [Place] wrapping `v`.
    pub fn wrap(v: impl Value) -> Place {
        Place(Rc::new(RefCell::new(v.into())))
    }
    /// Trade the contents of this [Place] with a new value.
    pub fn swap(&mut self, v: impl Value) -> Val {
        self.0.replace(v.into())
    }
    /// Update the contents of this [Place], discarding the old value.
    pub fn set(&mut self, v: impl Value) { self.swap(v); }

    /// Get a copy of the contained [Val].
    pub fn get(&self) -> Val { self.0.try_borrow().unwrap().clone() }
}


