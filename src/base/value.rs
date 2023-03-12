
use std::rc::Rc;
use std::any::{Any, TypeId};
use im_rc::HashMap;
use super::unique::Unique;

/// Metadata lookup for a value.
#[derive(Default, Clone)]
pub struct Meta {
    data: HashMap<Unique, Rc<dyn Any>>,
}

/// A reference-counted value, used directly by Worst programs.
/// Can be downcast into its original Rust value.
#[derive(Clone)]
pub struct Val {
    v: Rc<dyn Any>,
    meta: Meta,
}

/// Something that is, or could become, a [Val]
/// (e.g. to be given to an [Interpreter](crate::interpreter::Interpreter)).
pub trait Value: 'static {}

/// A [Val] but you know the type.
pub struct ValOf<T> {
    orig: Rc<dyn Any>,
    v: Rc<T>,
    modified: bool,
    meta: Meta,
}

impl<T> Clone for ValOf<T> {
    fn clone(&self) -> ValOf<T> {
        ValOf {
            orig: self.orig.clone(),
            v: self.v.clone(),
            modified: self.modified,
            meta: self.meta.clone(),
        }
    }
}

impl<T: Value> From<T> for Val {
    fn from(v: T) -> Val {
        Val::construct(v, Meta::default())
    }
}

impl Val {
    fn construct<T: Value>(v: T, meta: Meta) -> Self {
        Val { v: Rc::new(v), meta }
    }

    /// Get a reference to this value's Meta in order to query it and such.
    pub fn meta_ref(&self) -> &Meta { &self.meta }
    /// Update this value's metadata willy-nilly.
    /// Modifying the metadata won't affect other copies.
    pub fn meta_mut(&mut self) -> &mut Meta { &mut self.meta }

    /// Is the internal value of the given type?
    /// If so, the various downcasting functions should return correctly.
    pub fn is<T: Value>(&self) -> bool {
        self.v.is::<T>()
    }

    /// Get the TypeId of the contained value.
    pub fn val_type_id(&self) -> TypeId {
        (*self.v).type_id()
    }

    /// Get a reference to the inner value, if it is of the given type.
    pub fn downcast_ref<T: Value>(&self) -> Option<&T> {
        self.v.downcast_ref::<T>()
    }

    /// Turn this into a [ValOf] of the given type
    /// (or Err(self) with no changes).
    pub fn try_downcast<T: Value>(self) -> Result<ValOf<T>, Val> {
        ValOf::<T>::try_from(self)
    }
}

impl Meta {
    /// Get the number of Meta entries.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Get whether there are no entries.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get a reference to [T] associated with `u`,
    /// if `u` indeed refers to a [T].
    pub fn get_ref<T: 'static>(&self, u: &Unique) -> Option<&T> {
        self.data.get(u)
            .map(Rc::as_ref)
            .and_then(|r| r.downcast_ref::<T>())
    }

    /// Check whether the metadata contains the given [Unique].
    pub fn contains_val(&self, u: &Unique) -> bool {
        self.data.contains_key(u)
    }

    /// Get the [Val] associated with the given [Unique].
    /// Note that Meta entries are not [Val], so meta infomation is not retained.
    pub fn get_val(&self, u: &Unique) -> Option<Val> {
        self.data.get(u).map(|v| Val { v: v.clone(), meta: Meta::default() })
    }

    /// Insert a Val, discarding its metadata,
    /// and overwrite any previous value with the same type.
    pub fn insert_val(&mut self, u: Unique, v: Val) {
        self.data.insert(u, v.v);
    }

    /// Remove the [Val] associated with the given [Unique],
    /// and return whether it existed.
    pub fn remove_val(&mut self, u: &Unique) -> bool {
        self.data.remove(u).is_some()
    }
    /// Remove the [Val] associated with the given [Unique],
    /// and return it if it existed.
    pub fn take_val(&mut self, u: &Unique) -> Option<Val> {
        self.data.remove(u).map(|v| Val { v, meta: Meta::default() })
    }
}

impl<T: Value> ValOf<T> {
    /// Get a mutable reference to the meta.
    pub fn meta_mut(&mut self) -> &mut Meta { &mut self.meta }
    /// Get a reference to the meta.
    pub fn meta_ref(&self) -> &Meta { &self.meta }
}

impl<T: Value> From<T> for ValOf<T> {
    fn from(v: T) -> ValOf<T> {
        // for now
        Val::from(v).try_downcast::<T>().ok().unwrap()
    }
}


impl<T: Value> From<ValOf<T>> for Val {
    fn from(v: ValOf<T>) -> Val {
        if v.modified {
            Val { v: v.v, meta: v.meta, }
        } else {
            Val { v: v.orig, meta: v.meta, }
        }
    }
}

impl<T: Value> TryFrom<Val> for ValOf<T> {
    type Error = Val;
    fn try_from(this: Val) -> Result<ValOf<T>, Val> {
        let orig = this.v.clone();
        let meta = this.meta;
        match Rc::downcast::<T>(this.v) {
            Ok(v) => Ok(ValOf { orig, v, modified: false, meta, }),
            Err(v) => Err(Val { v, meta, }),
        }
    }
}

impl<T: Value> AsRef<T> for ValOf<T> {
    fn as_ref(&self) -> &T { &self.v }
}

impl<T: Value + Clone> AsMut<T> for ValOf<T> {
    // is this too much work for as_mut? should it be borrow_mut? who knows
    fn as_mut(&mut self) -> &mut T {
        self.modified = true;
        Rc::make_mut(&mut self.v)
    }
}

impl<T: Value + Clone> ValOf<T> {
    /// Take this apart to reveal its innards, discarding metadata.
    pub fn into_inner(self) -> T {
        Rc::try_unwrap(self.v).unwrap_or_else(|rc| (*rc).clone())
    }
}

