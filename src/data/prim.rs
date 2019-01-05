
use std::fmt;
use crate::data::value::*;
use crate::data::types::*;

impl StaticType for bool {
    fn static_type() -> Type {
        Type::new("bool")
    }
}
impl ValueDefaults for bool {}
impl Value for bool {}

impl StaticType for String {
    fn static_type() -> Type {
        Type::new("string")
    }
}

impl ValueShow for String {
    fn fmt_show(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, fmt)
    }
}

impl DefaultValueEq for String {}
impl DefaultValueHash for String {}
impl DefaultValueClone for String {}
impl ValueDebugDescribe for String {}
impl Value for String {}

impl StaticType for char {
    fn static_type() -> Type {
        Type::new("char")
    }
}

impl ValueShow for char {
    fn fmt_show(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, fmt)
    }
}

impl DefaultValueEq for char {}
impl DefaultValueHash for char {}
impl DefaultValueClone for char {}
impl ValueDebugDescribe for char {}
impl Value for char {}

