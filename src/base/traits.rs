
use std::fmt::Debug;

use crate::impl_value;
use super::value::*;

struct TypeName(String);
impl ImplValue for TypeName {}
/// Use in [impl_value] to specify the name of the type.
pub fn type_name(s: impl Into<String>) -> impl Value { TypeName(s.into()) }

struct DebugValue(Box<dyn Fn(&Val) -> String>);
impl_value!(DebugValue);
/// Use in [impl_value] to specify how to write members of the type to string.
pub fn value_tostring<T: 'static + ImplValue, F: 'static + Fn(&T) -> String>(f: F) -> impl Value {
    DebugValue(Box::new(move |v: &Val| f(v.downcast_ref::<T>().unwrap())))
}
/// Use in [impl_value] as a shorthand for [value_tostring] with [Debug].
pub fn value_debug<T: 'static + ImplValue + Debug>() -> impl Value {
    value_tostring(|v: &T| format!("{:?}", v))
}

struct EqValue(Box<dyn Fn(&Val, &Val) -> bool>);
impl_value!(EqValue);
/// Use in [impl_value] to use [eq](PartialEq::eq) instead of object identity
/// to check for equality between members of the type.
pub fn value_eq<T: 'static + ImplValue + PartialEq>() -> impl Value {
    EqValue(Box::new(move |a: &Val, b: &Val| {
        match (a.downcast_ref::<T>(), b.downcast_ref::<T>()) {
            (Some(a), Some(b)) => a == b,
            _ => false
        }
    }))
}

/// Meta value signalling whether the type or value it is attached to
/// represents some kind of error.
///
/// Set IsError on all members of a type:
/// ```ignore
/// struct BadSituation;
/// impl_value!(BadSituation, IsError);
/// assert!(IsError::is_error(&BadSituation));
/// ```
/// Set IsError on a single value:
/// ```ignore
/// let mut v = IsError::add("an error".to_string());
/// assert!(IsError::is_error(&v));
/// ```
pub struct IsError;
impl_value!(IsError);
impl IsError {
    /// Add IsError metadata to the value.
    pub fn add(v: impl Value) -> Val {
        let mut v: Val = v.into();
        v.meta_ref_mut().push(IsError);
        v
    }
    /// Check whether the value or its type is an error.
    pub fn is_error(v: &Val) -> bool {
        v.meta_ref().contains::<Self>() || v.type_meta().contains::<Self>()
    }
}

impl Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if let Some(dv) = self.type_meta().first_ref::<DebugValue>() {
            let d = dv.0(self);
            write!(f, "{}", d)?;
        } else if let Some(n) = self.type_meta().first_ref::<TypeName>() {
            write!(f, "<{}>", n.0)?;
        } else {
            write!(f, "<some value>")?;
        }
        Ok(())
    }
}

impl PartialEq for Val {
    fn eq(&self, you: &Self) -> bool {
        if self.identical(you) { return true; }
        if let Some(e) = self.type_meta().first_ref::<EqValue>() {
            e.0(self, you)
        } else if let Some(e) = you.type_meta().first_ref::<EqValue>() {
            e.0(you, self)
        } else { false }
    }
}
impl Eq for Val { }

