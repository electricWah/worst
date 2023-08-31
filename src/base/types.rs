
use std::cell::RefCell;
use std::rc::Rc;
use query_interface;
use super::unique::*;
use super::value::*;

impl Value for bool {}
impl Value for String {}
impl Value for i64 {}
impl Value for f64 {}
impl Value for Vec<u8> {} // bytevector

value!(Unique: dyn query_interface::ObjectHash, dyn query_interface::ObjectPartialEq);

/// Mutable memory location (a wrapper for [RefCell]).
#[derive(Clone)]
pub struct Place(Rc<RefCell<Val>>);
value!(Place);

impl Place {
    /// Create a new [Place] wrapping `v`.
    pub fn wrap(v: impl Into<Val>) -> Place {
        Place(Rc::new(RefCell::new(v.into())))
    }
    /// Trade the contents of this [Place] with a new value.
    pub fn swap(&mut self, v: impl Into<Val>) -> Val {
        self.0.replace(v.into())
    }
    /// Update the contents of this [Place], discarding the old value.
    pub fn set(&mut self, v: impl Into<Val>) { self.swap(v); }

    /// Get a copy of the contained [Val].
    pub fn get(&self) -> Val { self.0.try_borrow().unwrap().clone() }
}

/// Meta value signalling that the value represents some kind of error.
///
/// Set IsError on a single value:
/// ```ignore
/// let mut v = IsError::add("an error".to_string());
/// assert!(IsError::is_error(&v));
/// ```
pub struct IsError;
value!(IsError);

