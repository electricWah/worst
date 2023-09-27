
use std::fmt;

use super::value::*;

/// Symbol type: an unquoted word used to look up definitions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    v: String,
}
value!(Symbol: {Clone, Hash, PartialEq, Eq},
       dyn fmt::Debug,
       dyn fmt::Display);

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

