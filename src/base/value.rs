
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::any::{ Any, TypeId };

/// Metadata for [Val].
#[derive(Default, Debug, Clone)]
pub struct Meta(Vec<Val>);

/// A reference-counted value, used directly by Worst programs.
/// Can be downcast into its original Rust value.
#[derive(Clone)]
pub struct Val {
    v: Rc<dyn Any>,
    meta: Rc<Meta>,
}

/// Type value for vals for types. See [ImplValue].
#[derive(Debug)]
pub struct TypeVal(TypeId);
impl TypeVal {
    /// Get a TypeVal corresponding to the given type.
    pub fn of<T: 'static>() -> Self { TypeVal(TypeId::of::<T>()) }
}

/// The Worst value trait.
/// Usually not necessary to mention -
/// use [Value] in type parameters,
/// or [impl_value] to implement it.
///
/// This associates a [Val] with the implementing type to serve as a concrete
/// representation of the type for Worst.
pub trait ImplValue {
    thread_local! {
        /// A value shared by all instances of this type,
        /// so they can all have the same type within Worst.
        // This default value is for TypeVal specifically -
        // all other types should use impl_value
        static TYPE: RefCell<Val> =
            RefCell::new(Val::construct(TypeVal::of::<TypeVal>(),
                                        Rc::new(Meta::default())));
    }

    /// Add a new meta value. Take care to only do this once per `m`.
    /// The new value will only apply to newly-created instances of this type.
    // this could be a macro somehow?
    fn install_meta(m: impl Value) {
        Self::TYPE.with(|t| t.borrow_mut().meta_ref_mut().push(m))
    }
    /// Get the Worst type value for this type.
    fn get_type() -> Val {
        Self::TYPE.with(|t| t.borrow().clone())
    }
}
// This should be the only type using the default ImplValue,
// to prevent recursion-based headaches if this were to use impl_value.
impl ImplValue for TypeVal {}

/// Make a Rust type usable from Worst.
///
/// At its simplest, use [impl_value] to use Rust types from Worst:
/// ```ignore
/// struct Cool;
/// impl_value!(Cool);
/// // later, with an interpreter:
/// i.stack_push(Cool);
/// ```
/// You can use functions such as
/// [value_debug](crate::base::value_debug)
/// and [value_eq](crate::base::value_eq)
/// to act as "dynamic traits", designated [Meta] entries for the [Type]
/// which can wrap built-in traits, override default behaviour,
/// and let you pretend Worst has some kind of trait system.
/// ```ignore
/// #[derive(Debug)]
/// struct CoolDebuggable;
/// // type parameter needed because it's not particularly smart
/// impl_value!(CoolDebuggable, value_debug::<CoolDebuggable>());
/// ```
#[macro_export]
macro_rules! impl_value {
    ($t:ty) => { impl_value!($t,); };
    ($t:ty, $($m:expr),*) => {
        impl ImplValue for $t {
            thread_local! {
                static TYPE: std::cell::RefCell<Val> =
                          std::cell::RefCell::new(
                              Val::from(TypeVal::of::<$t>())
                              .with_meta(|m| {
                                  $(m.push($m);)*
                                  m.push(type_name(stringify!($t)));
                              }));
            }
        }
    }
}

/// Something that is, or could become, a [Val]
/// (e.g. to be given to an [Interpreter](crate::interpreter::Interpreter)).
pub trait Value: 'static + Into<Val> {}
impl<T: ImplValue + 'static> Value for T {}
impl Value for Val {}

impl<T: ImplValue + 'static> From<T> for Val {
    fn from(v: T) -> Val {
        T::TYPE.with(|t| {
            // Automatically add TypeVal for T to metadata.
            // Removing it will probably not break the Val too much
            // except maybe the things in base::traits.
            Val::construct(v, Rc::new(Meta::default().with(t.borrow().clone())))
        })
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
            dbg!(&self);
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

    /// Get a reference to this value's type.
    /// Option because type values themselves do not have a type.
    pub fn type_ref(&self) -> Option<&Val> {
        self.meta.first_ref_val::<TypeVal>()
    }
}

impl Meta {
    /// Add a new value.
    pub fn push(&mut self, v: impl Value) {
        self.0.push(v.into());
    }
    /// Add a new value, builder-style.
    pub fn with(mut self, v: impl Value) -> Self {
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

