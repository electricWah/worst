
use std::rc::Rc;
use std::any::Any;

/// A list of [Val] values. It is itself a [Value].
/// This is the primary container type in Worst.
/// It's a little like a Lisp list.
// defined here because the metadata for a Val is a List.
#[derive(Clone, Default)]
pub struct List {
    pub(crate) data: Vec<Val>,
}

/// A reference-counted value, used directly by Worst programs.
/// Can be downcast into its original Rust value.
#[derive(Clone)]
pub struct Val {
    v: Rc<dyn Any>,
    meta: Rc<List>,
}

/// Something that is, or could become, a [Val]
/// (e.g. to be given to an [Interpreter](crate::interpreter::Interpreter)).
pub trait Value: 'static {}
// impl Value for Val {}

/// A [Val] but you know the type.
pub struct ValOf<T> {
    orig: Rc<dyn Any>,
    v: Rc<T>,
    modified: bool,
    meta: Rc<List>,
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
        Val::construct(v, Rc::new(List::default()))
    }
}

impl Val {
    fn construct<T: Value>(v: T, meta: Rc<List>) -> Self {
        Val { v: Rc::new(v), meta }
    }

    /// Get a reference to this value's Meta in order to query it and such.
    pub fn meta_ref(&self) -> &List { &self.meta }
    /// Update this value's metadata willy-nilly.
    /// Modifying the metadata won't affect other copies.
    pub fn meta_mut(&mut self) -> &mut List {
        Rc::make_mut(&mut self.meta)
    }
    /// Builder-style wrapper for [meta_mut]
    pub fn with_meta(mut self, f: impl FnOnce(&mut List)) -> Self {
        f(self.meta_mut()); self
    }

    /// Is the internal value of the given type?
    /// If so, the various downcasting functions should return correctly.
    pub fn is<T: Value>(&self) -> bool {
        self.v.is::<T>()
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

impl<T: Value> ValOf<T> {
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

impl<T: Value> std::ops::Deref for ValOf<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.v
    }
}

impl<T: Value> AsRef<T> for ValOf<T> {
    fn as_ref(&self) -> &T {
        &*self.v
    }
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

