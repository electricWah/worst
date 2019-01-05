
use std::fmt;
use crate::data::types::*;
use crate::data::value::hash::*;
use downcast::*;

pub trait Value: Any + 'static + HasType + ValueEq + ValueClone + ValueHash + ValueShow + ValueDescribe {}
downcast!(Value);

pub struct BoxValue(pub Box<Value>);


pub trait ValueEq {
    fn equal(&self, _other: &Value) -> bool { false }
}

pub trait ValueClone {
    fn clone_value(&self) -> BoxValue;
}

/// Brief output for a value. Used in the REPL prompt.
/// Defaults to an "opaque" representation using the value's type.
pub trait ValueShow: HasType {
    fn fmt_show(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<{}>", self.type_of())
    }
}

/// Human-debuggable output. Used for detailed error messages.
pub trait ValueDescribe {
    fn fmt_describe(&self, fmt: &mut fmt::Formatter) -> fmt::Result;
}

pub trait ValueHash {
    fn can_hash_value(&self) -> bool { false }
    fn hash_value(&self, _state: &mut ValueHasher) { }
}


