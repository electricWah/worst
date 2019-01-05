
use std::fmt;
use std::hash::{Hash, Hasher};
use downcast::*;
use crate::data::types::*;

mod hash;
mod types;
mod defaults;

pub use self::hash::*;
pub use self::types::*;
pub use self::defaults::*;

impl BoxValue {
    pub fn new<V: Value>(v: V) -> Self {
        BoxValue(Box::new(v))
    }
    pub fn is_type<T: Value>(&self) -> bool {
        Downcast::<T>::is_type(&*self.0)
    }
    pub fn try_cast<T: Value + Sized>(self) -> Result<T, Type> {
        Downcast::<T>::downcast(self.0)
            .map_err(|d| d.into_object().type_of())
            .map(|v| *v)
    }
}

impl Clone for BoxValue {
    fn clone(&self) -> Self {
        self.0.clone_value()
    }
}

impl From<Box<Value>> for BoxValue {
    fn from(v: Box<Value>) -> BoxValue {
        BoxValue(v)
    }
}

// impl Into<Box<ValueFmtType>> for Box<Value> {
//     fn into(self) -> Box<ValueFmtType> {
//         self as Box<ValueFmtType>
//     }
// }
// impl Clone for BoxValue {
//     fn clone(&self) -> Self {
//         (*self.0).clone_value()
//     }
// }

// impl ValueFmtType for Box<Value> {
//     fn fmt_type(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         ValueFmtType::fmt_type(&*self, fmt)
//     }
// }

impl PartialEq for BoxValue {
    fn eq(&self, other: &Self) -> bool {
        self.0.equal(&*other.0)
    }
}
impl Eq for BoxValue {}

impl fmt::Debug for BoxValue {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt_describe(fmt)
    }
}

impl Hash for BoxValue {
    fn hash<H: Hasher>(&self, mut state: &mut H) {
        let mut hasher = ValueHasher(&mut state);
        self.0.hash_value(&mut hasher)
    }
}

// pub trait OpaqueValueDisplay {}
// impl<T: OpaqueValueDisplay + HasType> DisplayValue for T {
//     fn fmt_display(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         write!(fmt, "<{}>", self.type_of())
//     }
// }


