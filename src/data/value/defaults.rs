
use std::fmt;
use std::hash::Hash;
use crate::data::types::*;
use crate::data::value::types::*;
use crate::data::value::hash::*;

pub trait ValueDefaults {}
impl<T: ValueDefaults + Eq> DefaultValueEq for T {}
impl<T: ValueDefaults + Hash> DefaultValueHash for T {}
impl<T: ValueDefaults + Clone> DefaultValueClone for T {}
impl<T: ValueDefaults + fmt::Debug> ValueDebugDescribe for T {}
impl<T: ValueDefaults + fmt::Display> ValueDisplayShow for T {}

pub trait DefaultValueEq {}
impl<T: 'static + DefaultValueEq + Eq> ValueEq for T {
    fn equal(&self, other: &Value) -> bool {
        if let Ok(t) = other.downcast_ref::<T>() {
            self == t
        } else {
            false
        }
    }
}

pub trait DefaultValueClone {}
impl<T: 'static + DefaultValueClone + Clone + Value> ValueClone for T {
    fn clone_value(&self) -> BoxValue {
        BoxValue::new(self.clone())
    }
}

pub trait ValueDebugDescribe: fmt::Debug {}
impl<T: 'static + ValueDebugDescribe + fmt::Debug + Value> ValueDescribe for T {
    fn fmt_describe(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, fmt)
    }
}

pub trait ValueDisplayShow: fmt::Display {}
impl<T: ValueDisplayShow + fmt::Display + HasType> ValueShow for T {
    fn fmt_show(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

pub trait DefaultValueHash {}
impl<T: 'static + DefaultValueHash + Hash + Value> ValueHash for T {
    fn can_hash_value(&self) -> bool { true }
    fn hash_value(&self, state: &mut ValueHasher) {
        self.hash(state)
    }
}

