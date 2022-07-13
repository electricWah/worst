
//! The [Value] trait, [Val] type, and bits to work with Rust types from Worst.

use std::cell::RefCell;
use std::marker::PhantomData;
use std::fmt::{ Debug, Display };
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

/// Metadata record to be attached to a [Type] or individual [Val].
#[derive(Default, Debug, Clone)]
pub struct Meta(Vec<Val>);

/// Every [Val] has a Type, which determines how it works.
#[derive(Clone, Default, PartialEq)]
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
    pub fn try_read(v: Val) -> Option<Box<dyn std::io::Read>> {
        v.type_meta().first_val::<Self>()
            .map(|rv| rv.downcast_ref::<Self>().unwrap().0(v))
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
        if let Some(dv) = self.ty.meta.first::<DebugValue>() {
            let d = dv.0(self);
            write!(f, "{}", d)?;
        } else if let Some(n) = self.ty.meta.first::<TypeName>() {
            write!(f, "<{}>", n.0)?;
        } else {
            write!(f, "<some value>")?;
        }
        Ok(())
    }
}

impl PartialEq for Val {
    fn eq(&self, you: &Self) -> bool {
        if Rc::ptr_eq(&self.v, &you.v) { return true; }
        if let Some(e) = self.ty.meta.first::<EqValue>() {
            e.0(self, you)
        } else if let Some(e) = you.ty.meta.first::<EqValue>() {
            e.0(you, self)
        } else { false }
    }
}
impl Eq for Val { }

impl Val {
    fn new_type<T: Value>(v: T, ty: Type) -> Self {
        Val { v: Rc::new(v), meta: Rc::new(Meta::default()), ty }
    }
    /// If the inner value is a T, take it.
    /// If there are multiple references, it is cloned.
    ///
    /// Not recommended as this loses metadata.
    pub fn downcast<T: Value + Clone>(self) -> Option<T> {
        if self.v.is::<T>() {
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

    ///// If the inner value is a T, get a mutable reference to it.
    ///// It is cloned if there are other references to the value.
    /////
    ///// This can be more efficient than unconditionally cloning the inner value.
    //pub fn downcast_mut<T: Value + Clone>(&mut self) -> Option<&mut T> {
    //    match Rc::downcast(self.v.clone()) {
    //        Ok(mut v) => Some(Rc::make_mut(&mut v)),
    //        Err(_) => None,
    //    }
    //}
    /// If this is the only reference to its inner value,
    /// and it's a T, get a mutable reference to it.
    pub fn try_downcast_mut<T: Value>(&mut self) -> Option<&mut T> {
        if let Some(v) = Rc::get_mut(&mut self.v) {
            v.downcast_mut::<T>()
        } else {
            None
        }
    }

    pub fn is<T: Value>(&self) -> bool {
        self.v.is::<T>()
    }
    pub fn type_id(&self) -> TypeId {
        self.v.type_id()
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

impl<T: Value> TryFrom<Val> for Vals<T> {
    type Error = Val;
    fn try_from(v: Val) -> Result<Vals<T>, Val> {
        if v.is::<T>() { Ok(Vals { inner: vec![v], ty: PhantomData }) }
        else { Err(v) }
    }
}

/// Symbol type: an unquoted word used to look up definitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    v: String,
}
impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.v)
    }
}
impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str { self.v.as_ref() }
}
/// Conversion into a symbol.
///
/// May be removed in favour of [Symbol::from].
pub trait ToSymbol { fn to_symbol(self) -> Symbol; }
impl<T: Into<Symbol>> ToSymbol for T {
    fn to_symbol(self) -> Symbol { self.into() }
}

impl From<&str> for Symbol {
    fn from(s: &str) -> Symbol { Symbol { v: s.to_string() } }
}
impl From<String> for Symbol {
    fn from(v: String) -> Symbol { Symbol { v } }
}
impl From<Symbol> for String {
    fn from(s: Symbol) -> Self { s.v }
}

impl_value!(Symbol, value_eq::<Symbol>(), value_tostring(Symbol::to_string), type_name("symbol"));
fn bool_tostring(b: &bool) -> String { (if *b { "#t" } else { "#f" }).into() }
impl_value!(bool, value_eq::<bool>(), value_tostring(bool_tostring));
impl_value!(String, value_eq::<String>(), value_debug::<String>(), type_name("string"));
// NOTE always use String instead
// impl_value!(&'static str, type_name("string"));

// TODO bunch of numbers, better than this
impl_value!(i32, value_eq::<i32>(), value_debug::<i32>(), type_name("number"));
impl_value!(i64, value_eq::<i64>(), value_debug::<i64>(), type_name("number"));
impl_value!(f64, value_eq::<f64>(), value_debug::<f64>(), type_name("number"));

/// Mutable memory location (a wrapper for [RefCell]).
#[derive(Clone, Eq)]
pub struct Place(Rc<RefCell<Val>>);
impl PartialEq for Place {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl_value!(Place, type_name("place"));

impl Place {
    /// Create a new [Place] wrapping `v`.
    pub fn wrap(v: impl Value) -> Place {
        Place(Rc::new(RefCell::new(v.into())))
    }
    /// Trade the contents of this [Place] with a new value.
    pub fn swap(&mut self, v: impl Value) -> Val {
        self.0.replace(v.into())
    }
    /// Update the contents of this [Place], discarding the old value.
    pub fn set(&mut self, v: impl Value) { self.swap(v); }

    /// Get a copy of the contained [Val].
    pub fn get(&self) -> Val { self.0.try_borrow().unwrap().clone() }
}

