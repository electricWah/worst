
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::any::{ Any, TypeId };

/// The Worst value trait.
/// Usually use [Value] or [impl_value] instead of this directly.
pub trait ImplValue {
    thread_local!(static TYPE: RefCell<Type> = RefCell::new(Type::default()));

    /// Add a new type-meta value. Take care to only do this once per `m`.
    /// The new value will only apply to newly-created instances of this type.
    // (for now - otherwise make it Rc<RefCell<>>
    // this could be a macro somehow?
    fn install_meta(m: impl Value) {
        Self::TYPE.with(|t| t.borrow_mut().push_meta(m.into()))
    }
    fn get_type() -> Type {
        Self::TYPE.with(|t| t.borrow().clone())
    }
}

/// Make a Rust type usable from Worst.
///
/// At its simplest, use [impl_value] to use Rust types from Worst:
/// ```ignore
/// struct Cool;
/// impl_value!(Cool);
/// // later, with an interpreter:
/// i.stack_push(Cool);
/// ```
/// You can use functions such as [value_debug] and [value_eq]
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
            thread_local!(static TYPE: std::cell::RefCell<Type> =
                          std::cell::RefCell::new(
                                  Type::new_meta::<$t>(Meta::default()
                                            $(.with($m))*
                                            .with(type_name(stringify!($t)))
                                           )));
        }
    }
}

/// Something that is, or could become, a [Val]
/// (e.g. to be given to an [Interpreter](crate::interpreter::Interpreter)).
pub trait Value: 'static + Into<Val> {}
impl<T: ImplValue + 'static> Value for T {}

/// Every [Val] has a Type, which determines how it works.
#[derive(Clone, Default)]
pub struct Type {
    // option: default T::TYPE requires a value, but doesn't have access to T
    id: Option<TypeId>,
    meta: Rc<Meta>,
}
impl Type {
    pub fn new_meta<T: 'static>(m: Meta) -> Self {
        Type { id: Some(TypeId::of::<T>()), meta: Rc::new(m) }
    }
    pub fn push_meta(&mut self, m: Val) {
        Rc::make_mut(&mut self.meta).0.push(m)
    }
}
impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        // TODO this might be asking for trouble - just use derive?
        self.id == other.id
    }
}
// TODO a Val wrapper for Type (don't use Type directly)
// impl ImplValue for Type {}

/// A reference-counted value, used directly by Worst programs.
/// Can be downcast into its original Rust value.
#[derive(Clone)]
pub struct Val {
    v: Rc<dyn Any>,
    meta: Rc<Meta>,
    ty: Type,
}
impl Value for Val {}

impl<T: ImplValue + 'static> From<T> for Val {
    fn from(v: T) -> Val {
        T::TYPE.with(|t| Val::new_type(v, t.borrow().clone()))
    }
}

impl Val {
    fn new_type<T: Value>(v: T, ty: Type) -> Self {
        Val { v: Rc::new(v), meta: Rc::new(Meta::default()), ty }
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
    pub fn try_set<T: Value>(&mut self, v: impl Into<Rc<T>>) -> bool {
        if !self.is::<T>() { return false; }
        self.v = v.into() as Rc<dyn Any>;
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
    /// More exact and a bit faster than [eq], but not as useful.
    pub fn identical(&self, ye: &Self) -> bool {
        Rc::ptr_eq(&self.v, &ye.v)
    }

    pub fn is<T: Value>(&self) -> bool {
        self.v.is::<T>()
    }

    pub fn meta_ref(&self) -> &Meta { &self.meta }
    pub fn meta_ref_mut(&mut self) -> &mut Meta {
        Rc::make_mut(&mut self.meta)
    }
    pub fn with_meta(mut self, v: impl Value) -> Self {
        self.meta_ref_mut().push(v); self
    }

    pub fn type_ref(&self) -> &Type { &self.ty }
    pub fn type_meta(&self) -> &Meta { &self.ty.meta }
    // pub fn method<T: Value>(&self) -> Option<&T> {
    //     if let Some(ty) = self.meta.first::<Type>() {
    //         ty.1.first::<T>()
    //     } else { None }
    // }
}

/// Metadata record to be attached to a [Type] or individual [Val].
#[derive(Default, Debug, Clone)]
pub struct Meta(Vec<Val>);

impl Meta {
    /// Add a new value.
    pub fn push(&mut self, v: impl Value) {
        self.0.push(v.into());
    }
    /// Add a new value, builder-style.
    pub fn with(mut self, v: impl Value) -> Self {
        self.push(v); self
    }
    /// Find the first `T`.
    pub fn first<T: Value>(&self) -> Option<&T> {
        self.0.iter().find_map(|v| v.downcast_ref::<T>())
    }
    /// Find the first `T` and copy it as a [Val] (preserving its metadata).
    pub fn first_val<T: Value>(&self) -> Option<Val> {
        self.0.iter().find(|v| v.is::<T>()).cloned()
    }
    /// Check if this contains a `T`.
    pub fn contains<T: Value>(&self) -> bool {
        self.0.iter().any(|v| v.is::<T>())
    }
}

