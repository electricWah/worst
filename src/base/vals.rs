
use std::marker::PhantomData;

use super::value::*;

/// A typed wrapper for one or more [Val] (currently only one).
pub struct Vals<T> {
    inner: Vec<Val>,
    ty: PhantomData<T>,
}

impl<T: Value> AsRef<T> for Vals<T> {
    fn as_ref(&self) -> &T {
        self.inner[0].downcast_ref().unwrap()
    }
}

impl<T: Value + Clone> AsMut<T> for Vals<T> {
    fn as_mut(&mut self) -> &mut T {
        self.inner[0].try_downcast_mut().unwrap()
    }
}

impl<T: Value> TryFrom<Val> for Vals<T> {
    type Error = Val;
    fn try_from(v: Val) -> Result<Vals<T>, Val> {
        if v.is::<T>() { Ok(Vals { inner: vec![v], ty: PhantomData }) }
        else { Err(v) }
    }
}

impl<T> Vals<T> {
    pub fn into_val(self) -> Val {
        // can't inner[0], also, think about multi-vals
        self.inner.into_iter().next().unwrap()
    }
}

impl<T: Value + Clone> Vals<T> {
    pub fn unwrap(self) -> T {
        self.inner.into_iter().next().unwrap().downcast().unwrap()
    }
}

