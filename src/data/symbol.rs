
use std::fmt;
use std::borrow::*;
use data::value::*;
use data::types::*;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Symbol(pub String);


impl<'a> From<&'a str> for Symbol {
    fn from(v: &'a str) -> Symbol {
        Symbol(v.to_string())
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Into<String> for Symbol {
    fn into(self) -> String {
        self.0
    }
}

impl Borrow<String> for Symbol {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl BorrowMut<String> for Symbol {
    fn borrow_mut(&mut self) -> &mut String {
        &mut self.0
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

