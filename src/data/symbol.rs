
use std::fmt;
use crate::data::value::*;
use crate::data::types::*;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Symbol(String);

impl Symbol {
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl<'a> From<&'a str> for Symbol {
    fn from(v: &'a str) -> Symbol {
        Symbol(v.to_string())
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl StaticType for Symbol {
    fn static_type() -> Type {
        Type::new("symbol")
    }
}

impl ValueDescribe for Symbol {
    fn fmt_describe(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}
impl ValueShow for Symbol {
    fn fmt_show(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

// impl HasType for Symbol {
//     fn get_type(&self) -> &Type {
//         Self::get_type()
//     }
// }
impl DefaultValueEq for Symbol {}
impl DefaultValueHash for Symbol {}
impl DefaultValueClone for Symbol {}
impl Value for Symbol {}

