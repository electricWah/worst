
use std::rc::Rc;
use std::any::Any;

/// Metadata for [Val].
#[derive(Default, Clone)]
pub struct Meta(Vec<Val>);

/// A reference-counted value, used directly by Worst programs.
/// Can be downcast into its original Rust value.
#[derive(Clone)]
pub struct Val {
    v: Rc<dyn Any>,
    meta: Rc<Meta>,
}

/// Something that is, or could become, a [Val]
/// (e.g. to be given to an [Interpreter](crate::interpreter::Interpreter)).
pub trait Value: 'static {}
// impl Value for Val {}

impl<T: Value> From<T> for Val {
    fn from(v: T) -> Val {
        Val::construct(v, Rc::new(Meta::default()))
    }
}

impl Val {
    fn construct<T: Value>(v: T, meta: Rc<Meta>) -> Self {
        Val { v: Rc::new(v), meta }
    }
    /// If the inner value is a T, take it.
    /// If there are multiple references, it is cloned.
    ///
    /// Not recommended as this loses metadata.
    pub fn downcast<T: Value + Clone>(self) -> Option<T> {
        if self.is::<T>() {
            // Rc::make_mut(&mut self.v);
            Some(Rc::try_unwrap(Rc::downcast::<T>(self.v).unwrap())
                 .unwrap_or_else(|rc| (*rc).clone()))
        } else {
            None
        }
    }
    /// If the inner value is a T, get a reference to it.
    pub fn downcast_ref<T: Value>(&self) -> Option<&T> {
        self.v.downcast_ref::<T>()
    }

    /// If the inner value is a T, get an Rc of it
    /// which shares the same location as the inner value.
    pub fn downcast_rc<T: Value>(&self) -> Option<Rc<T>> {
        Rc::downcast::<T>(self.v.clone()).ok()
    }

    /// If the inner value is a T, overwrite it with the given new value.
    /// Returns whether it succeeded.
    pub fn try_set<T: Value>(&mut self, v: Rc<T>) -> bool {
        if !self.is::<T>() { return false; }
        self.v = v as Rc<dyn Any>;
        true
    }

    /// If this is the only reference to its inner value,
    /// and it's a T, get a mutable reference to it.
    pub fn try_downcast_mut<T: Value>(&mut self) -> Option<&mut T> {
        if let Some(v) = Rc::get_mut(&mut self.v) {
            v.downcast_mut::<T>()
        } else {
            None
        }
    }

    /// Whether this and that have the very same memory location.
    /// More exact and a bit faster than [eq](Val::eq), but not as useful.
    pub fn identical(&self, ye: &Self) -> bool {
        Rc::ptr_eq(&self.v, &ye.v)
    }

    /// Is the internal value of the given type?
    /// If so, the various downcasting functions should return correctly.
    pub fn is<T: Value>(&self) -> bool {
        self.v.is::<T>()
    }

    /// Get a reference to this value's Meta in order to query it and such.
    pub fn meta_ref(&self) -> &Meta { &self.meta }
    /// Update this value's metadata willy-nilly.
    /// Modifying the metadata won't affect other copies.
    pub fn meta_ref_mut(&mut self) -> &mut Meta {
        Rc::make_mut(&mut self.meta)
    }
    /// Builder-style wrapper for [meta_ref_mut]
    pub fn with_meta(mut self, f: impl FnOnce(&mut Meta)) -> Self {
        f(self.meta_ref_mut()); self
    }
}

impl Meta {
    /// Add a new value.
    pub fn push(&mut self, v: impl Into<Val>) {
        self.0.push(v.into());
    }
    /// Add a new value, builder-style.
    pub fn with(mut self, v: impl Into<Val>) -> Self {
        self.push(v); self
    }

    /// Find the first `T` and get a reference to its value.
    pub fn first_ref_val<T: Value>(&self) -> Option<&Val> {
        self.0.iter().rev().find(|v| v.is::<T>())
    }

    /// Find the first `T`.
    pub fn first_ref<T: Value>(&self) -> Option<&T> {
        self.0.iter().rev().find_map(|v| v.downcast_ref::<T>())
    }
    /// Check if this contains a `T`.
    pub fn contains<T: Value>(&self) -> bool {
        self.0.iter().rev().any(|v| v.is::<T>())
    }

    /// Find the first `T` as a [Val] and remove it.
    pub fn take_first_val<T: Value>(&mut self) -> Option<Val> {
        if let Some(idx) = self.0.iter().position(|v| v.is::<T>()) {
            Some(self.0.remove(idx))
        } else { None }
    }

    /// Get an iterator over the values.
    pub fn iter(&self) -> impl Iterator<Item=&Val> {
        self.0.iter()
    }
}

