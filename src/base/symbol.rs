
use std::fmt::{ Debug, Display };
use std::hash;
use std::collections::hash_map::DefaultHasher;

use super::value::*;

/// Symbol type: an unquoted word used to look up definitions.
/// Symbols are pre-hashed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    v: String,
    hash: u64,
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

impl From<Symbol> for String {
    fn from(s: Symbol) -> Self { s.v }
}

impl From<String> for Symbol {
    fn from(v: String) -> Symbol {
        let mut h = DefaultHasher::new();
        hash::Hash::hash(&v, &mut h);
        let hash = hash::Hasher::finish(&h);
        Symbol { v, hash }
    }
}
impl From<&str> for Symbol {
    fn from(s: &str) -> Symbol { Symbol::from(s.to_string()) }
}

impl Value for Symbol {}

// impl hash::Hash for Symbol {
//     fn hash<T: hash::Hasher>(&self, h: &mut T) {
//         h.write_u64(self.hash);
//     }
// }

