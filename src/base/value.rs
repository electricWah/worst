
use std::fmt;
use std::any::Any;
use super::unique::Unique;

use im_rc::HashMap;
use query_interface;
use downcast_rs;

/// Metadata lookup for a value.
#[derive(Default, Clone, Debug)]
pub struct Meta {
    data: HashMap<Unique, Box<dyn Value>>,
}

/// A thingy used directly by Worst programs, with added metadata.
/// Can be downcast into its original Rust value.
#[derive(Clone, Debug)]
pub struct Val {
    v: Box<dyn Value>,
    meta: Meta,
}

/// Something that is, or could become, a [Val]
/// (e.g. to be given to an [Interpreter](crate::interpreter::Interpreter)).
pub trait Value: 'static + query_interface::Object + downcast_rs::Downcast {}
mod suppress_missing_docs_hack {
    #![allow(missing_docs)]
    query_interface::mopo!(dyn super::Value);
}
downcast_rs::impl_downcast!(Value);

macro_rules! value {
    (@expand Clone) => (dyn query_interface::ObjectClone);
    (@expand Hash) => (dyn query_interface::ObjectHash);
    (@expand PartialEq) => (dyn query_interface::ObjectPartialEq);
    (@expand Eq) => (dyn query_interface::ObjectEq);
    (@expand Ord) => (dyn query_interface::ObjectOrd);
    (@expand PartialOrd) => (dyn query_interface::ObjectPartialOrd);
    ($t:ty) => {
        impl $crate::base::Value for $t {}
        interfaces!($t: dyn $crate::base::Value);
    };
    ($t:ty: {$($x:ident),*}) => {
        impl $crate::base::Value for $t {}
        interfaces!($t: dyn $crate::base::Value,
                    $(value!(@expand $x)),*);
    };
    ($t:ty: $({$($x:ident),*},)? $($r:ty),*) => {
        impl $crate::base::Value for $t {}
        interfaces!($t: dyn $crate::base::Value,
                    $($(value!(@expand $x),)*)?
                    $($r),*);
    };
}
pub(crate) use value; // TODO public everywhere

// need to wrap TypeId because it doesn't already implement Object
/// Simple wrapper for TypeId that implements [Value].
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct TypeId(pub std::any::TypeId);
value!(TypeId: {Clone},
        dyn query_interface::ObjectHash,
        dyn query_interface::ObjectPartialEq,
        dyn std::fmt::Debug,
        dyn fmt::Display);

impl TypeId {
    /// Forwards to [std::any::TypeId::of].
    pub fn of<T: 'static>() -> Self { TypeId(std::any::TypeId::of::<T>()) }
}
impl fmt::Display for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<type-id>")
    }
}

/// A [Val] with a known type.
#[derive(Clone, Debug)]
pub struct ValOf<T> {
    v: Box<T>,
    meta: Meta,
}

impl<T: Value> From<T> for Val {
    fn from(v: T) -> Val {
        Val { v: Box::new(v), meta: Meta::default() }
    }
}

impl Val {
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
        TypeId((*self.v).type_id())
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

    /// Get a reference to a trait object that the contained type implements.
    pub fn as_trait_ref<T: Any + ?Sized>(&self) -> Option<&T> {
        self.v.as_ref().query_ref::<T>()
    }

    /// Get a mutable reference to a trait object that the contained type implements.
    /// Returns [None] if the value doesn't implement [T],
    /// or if it wasn't unique and can't be cloned.
    pub fn as_trait_mut<T: Any + ?Sized>(&mut self) -> Option<&mut T> {
        self.v.query_mut::<T>()
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
    pub fn get_ref<T: Value>(&self, u: &Unique) -> Option<&T> {
        self.data.get(u)
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
        ValOf { v: Box::new(v), meta: Meta::default(), }
    }
}


impl<T: Value> From<ValOf<T>> for Val {
    fn from(v: ValOf<T>) -> Val {
        Val { v: Box::new(*v.v), meta: v.meta, }
    }
}

impl<T: Value> TryFrom<Val> for ValOf<T> {
    type Error = Val;
    fn try_from(this: Val) -> Result<ValOf<T>, Val> {
        let meta = this.meta;
        match this.v.downcast::<T>() {
            Ok(v) => Ok(ValOf { v, meta, }),
            Err(v) => Err(Val { v, meta, }),
        }
    }
}

impl<T: Value> AsRef<T> for ValOf<T> {
    fn as_ref(&self) -> &T { self.v.as_ref() }
}

impl<T: Value> AsMut<T> for ValOf<T> {
    fn as_mut(&mut self) -> &mut T { self.v.as_mut() }
}

impl<T: Value> ValOf<T> {
    /// Take this apart to reveal its innards, discarding metadata.
    pub fn into_inner(self) -> T { *self.v }
}

