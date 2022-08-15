
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

/// Meta value for [Read](std::io::Read) values.
pub struct ReadValue(Box<dyn Fn(Val) -> Box<dyn std::io::Read>>);
impl_value!(ReadValue);
/// Use in [impl_value] to show that members of the type implement [Read](std::io::Read).
///
/// This requires [Clone] since it uses [Val::downcast].
/// Consequently, types implementing this will probably be reference-counted.
/// ```ignore
/// struct MyReadableThing;
/// impl std::io::Read for MyReadableThing { /* ... */ }
/// impl_value!(MyReadableThing, value_read::<MyReadableThing>());
/// ```
pub fn value_read<T: 'static + Clone + ImplValue + std::io::Read>() -> impl Value {
    ReadValue(Box::new(|a: Val| Box::new(a.downcast::<T>().unwrap())))
}
impl ReadValue {
    /// Try and get a mutable [Read](std::io::Read) out of the value
    /// (see [value_read]).
    /// On failure, return the input value unscathed.
    pub fn try_read(v: Val) -> Result<Box<dyn std::io::Read>, Val> {
        if let Some(rv) = v.clone().type_meta().first_ref::<Self>() {
            Ok(rv.0(v))
        } else { Err(v) }
    }
    /// Check if the [Val] implements [Read](std::io::Read) (see [value_read]).
    pub fn can(v: &Val) -> bool { v.type_meta().contains::<Self>() }
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
/// let mut v = IsError::set("an error".to_string());
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

