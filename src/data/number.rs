
use std::fmt;

use num_traits::cast::NumCast;

use crate::data::value::*;
use crate::data::types::*;
use crate::data::error::*;

pub trait Numeric: Sized {
    fn cast<T: NumCast>(self) -> Result<T, ConversionFailure>;
    fn from_num<T: NumCast>(t: T) -> Result<Self, ConversionFailure>;
}

impl Numeric for isize {
    fn cast<T: NumCast>(self) -> Result<T, ConversionFailure> {
        match NumCast::from(self) {
            Some(v) => Ok(v),
            None => Err(ConversionFailure)
        }
    }
    fn from_num<T: NumCast>(t: T) -> Result<Self, ConversionFailure> {
        match NumCast::from(t) {
            Some(v) => Ok(v),
            None => Err(ConversionFailure)
        }
    }
}

impl StaticType for isize {
    fn static_type() -> Type {
        Type::new("int")
    }
}

impl ValueShow for isize {
    fn fmt_show(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, fmt)
    }
}

impl DefaultValueEq for isize {}
impl DefaultValueHash for isize {}
impl DefaultValueClone for isize {}
impl ValueDebugDescribe for isize {}
impl Value for isize {}

impl StaticType for f64 {
    fn static_type() -> Type {
        Type::new("float")
    }
}

impl ValueShow for f64 {
    fn fmt_show(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, fmt)
    }
}

impl ValueHash for f64 {
    fn can_hash_value(&self) -> bool {
        false
    }
}

impl ValueEq for f64 {
    fn equal(&self, other: &Value) -> bool {
        if let Ok(t) = other.downcast_ref::<f64>() {
            self == t
        } else {
            false
        }
    }
}

impl DefaultValueClone for f64 {}
impl ValueDebugDescribe for f64 {}
impl Value for f64 {}

