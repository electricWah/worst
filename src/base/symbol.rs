
use std::fmt::{ Debug, Display };

use super::value::*;

/// Symbol type: an unquoted word used to look up definitions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    v: String,
}
impl Value for Symbol {}

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

impl From<Symbol> for String {
    fn from(s: Symbol) -> Self { s.v }
}

impl From<String> for Symbol {
    fn from(v: String) -> Symbol { Symbol { v } }
}
impl From<&str> for Symbol {
    fn from(s: &str) -> Symbol { Symbol::from(s.to_string()) }
}

