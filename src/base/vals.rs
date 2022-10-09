
use std::rc::Rc;
use super::value::*;

// implementing AsMut by prodding Val means cloning its Rc and having to modify
// its clone, so, grab a clone at the start, keep it typed for less unwrapping,
// and write it to the val when converting if required
#[derive(Clone)]
struct OneVal<T> {
    rc: Rc<T>,
    val: Val,
    modified: bool,
}

/// A typed wrapper for one or more [Val] (currently only one).
pub struct Vals<T> {
    inner: OneVal<T>,
}

impl<T: Value> AsRef<T> for Vals<T> {
    fn as_ref(&self) -> &T {
        &*self.inner.rc
    }
}

impl<T: Value + Clone> AsMut<T> for Vals<T> {
    // is this too much work for as_mut? should it be borrow_mut? who knows
    fn as_mut(&mut self) -> &mut T {
        self.inner.modified = true;
        Rc::make_mut(&mut self.inner.rc)
    }
}

impl<T: Value> TryFrom<Val> for Vals<T> {
    type Error = Val;
    fn try_from(val: Val) -> Result<Vals<T>, Val> {
        if let Some(rc) = val.downcast_rc() {
            Ok(Vals { inner: OneVal { rc, val, modified: false, } })
        } else {
            Err(val)
        }
    }
}

impl<T: Value> Vals<T> {
    /// Get an untyped Val out, like [into] but non-consuming.
    pub fn get_val(&self) -> Val {
        if self.inner.modified {
            let mut val = self.inner.val.clone();
            val.try_set(self.inner.rc.clone());
            val
        } else {
            self.inner.val.clone()
        }
    }
}

impl<T: Value> From<Vals<T>> for Val {
    fn from(v: Vals<T>) -> Val {
        if v.inner.modified {
            let mut val = v.inner.val;
            val.try_set(v.inner.rc);
            val
        } else {
            v.inner.val
        }
    }
}

impl<T: Value + Clone> Vals<T> {
    /// Take this apart to reveal its innards, discarding metadata.
    pub fn into_inner(self) -> T {
        Rc::try_unwrap(self.inner.rc).unwrap_or_else(|rc| (*rc).clone())
    }
}

