
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::rc::Rc;
use std::any::Any;

pub trait ImplValue {
    thread_local!(static TYPE: RefCell<Rc<Type>> =
                  RefCell::new(Rc::new(Type(Meta::default()))));

    /// Add a new type-meta value. Take care to only do this once!
    /// The new value will only apply to newly-created instances of this type.
    // (for now - otherwise make it Rc<RefCell<>>
    // this could be a macro somehow?
    fn install_meta(m: impl Value) {
        Self::TYPE.with(|t| Rc::make_mut(&mut t.borrow_mut()).0.0.push(m.into()))
    }
}

#[macro_export]
macro_rules! impl_value {
    ($t:ty) => { impl_value!($t,); };
    ($t:ty, $($m:expr),*) => {
        impl ImplValue for $t {
            thread_local!(static TYPE: std::cell::RefCell<std::rc::Rc<Type>> =
                          std::cell::RefCell::new(
                              std::rc::Rc::new(
                                  Type::new(Meta::default()
                                            $(.with($m))*
                                            .with(type_name(stringify!($t)))
                                           ))));
        }
    }
}

pub trait Value: 'static + Into<Val> {}
impl<T: ImplValue + 'static> Value for T {}

#[derive(Default, Debug, Clone)]
pub struct Meta(Vec<Val>);

#[derive(Clone)]
pub struct Type(Meta);
impl ImplValue for Type {}
impl Type {
    pub fn new(m: Meta) -> Self { Type(m) }
}

#[derive(Clone)]
pub struct Val {
    v: Rc<Box<dyn Any>>,
    meta: Rc<Meta>,
    ty: Rc<Type>,
}
impl Value for Val {}

impl<T: ImplValue + 'static> From<T> for Val {
    fn from(v: T) -> Val {
        T::TYPE.with(|t| Val::new_type(v, t.borrow().clone()))
    }
}

struct TypeName(String);
impl ImplValue for TypeName {}
pub fn type_name(s: impl Into<String>) -> impl Value { TypeName(s.into()) }

struct DebugValue(Box<dyn Fn(&Val) -> String>);
impl_value!(DebugValue);
pub fn value_tostring<T: 'static + ImplValue, F: 'static + Fn(&T) -> String>(f: F) -> impl Value {
    DebugValue(Box::new(move |v: &Val| f(&v.downcast_ref::<T>().unwrap())))
}
pub fn value_debug<T: 'static + ImplValue + Debug>() -> impl Value {
    value_tostring(|v: &T| format!("{:?}", v))
}

struct EqValue(Box<dyn Fn(&Val, &Val) -> bool>);
impl_value!(EqValue);
pub fn value_eq<T: 'static + ImplValue + PartialEq>() -> impl Value {
    EqValue(Box::new(move |a: &Val, b: &Val| {
        match (a.downcast_ref::<T>(), b.downcast_ref::<T>()) {
            (Some(a), Some(b)) => a == b,
            _ => false
        }
    }))
}

pub struct ReadValue(Box<dyn Fn(&mut Val) -> &mut dyn std::io::Read>);
impl_value!(ReadValue);
pub fn value_read<T: 'static + ImplValue + std::io::Read>() -> impl Value {
    ReadValue(Box::new(|a: &mut Val| a.downcast_mut::<T>().unwrap()))
}
impl ReadValue {
    pub fn try_read<'a>(v: &'a mut Val) -> Option<impl BorrowMut<dyn std::io::Read + 'a>> {
        v.type_meta().first_val::<Self>()
            .map(|rv| rv.downcast_ref::<Self>().unwrap().0(v))
    }
    pub fn can(v: &Val) -> bool { v.type_meta().contains::<Self>() }
}

impl Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if let Some(dv) = self.ty.0.first::<DebugValue>() {
            let d = dv.0(self);
            write!(f, "{}", d)?;
        } else if let Some(n) = self.ty.0.first::<TypeName>() {
            write!(f, "<{}>", n.0)?;
        } else {
            write!(f, "{}", "<some value>")?;
        }
        Ok(())
    }
}

impl PartialEq for Val {
    fn eq(&self, you: &Self) -> bool {
        if Rc::ptr_eq(&self.v, &you.v) { return true; }
        if let Some(e) = self.ty.0.first::<EqValue>() {
            e.0(&self, you)
        } else if let Some(e) = you.ty.0.first::<EqValue>() {
            e.0(you, &self)
        } else { false }
    }
}
impl Eq for Val { }

impl Val {
    fn new_type<T: Value>(v: T, ty: Rc<Type>) -> Self {
        Val { v: Rc::new(Box::new(v)), meta: Rc::new(Meta::default()), ty }
    }
    pub fn downcast<T: Value + Clone>(self) -> Result<T, Val> {
        match Rc::try_unwrap(self.v) {
            Ok(v) => {
                match v.downcast::<T>() {
                    Ok(v) => Ok(*v),
                    Err(v) => Err(Val { v: Rc::new(v), meta: self.meta, ty: self.ty }),
                }
            },
            Err(v) => {
                if let Some(v) = v.downcast_ref::<T>() {
                    Ok(v.clone())
                } else {
                    Err(Val { v, meta: self.meta, ty: self.ty })
                }
            },
        }
    }
    pub fn downcast_ref<T: Value>(&self) -> Option<&T> {
        self.v.downcast_ref::<T>()
    }
    pub fn downcast_mut<T: Value>(&mut self) -> Option<&mut T> {
        if let Some(v) = Rc::get_mut(&mut self.v) {
            v.downcast_mut::<T>()
        } else {
            None
            //Rc::make_mut(&mut self.v).downcast_mut::<T>()
        }
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

    pub fn type_meta(&self) -> &Meta { &self.ty.0 }
    // pub fn method<T: Value>(&self) -> Option<&T> {
    //     if let Some(ty) = self.meta.first::<Type>() {
    //         ty.1.first::<T>()
    //     } else { None }
    // }
}

impl Meta {
    fn push(&mut self, v: impl Value) {
        self.0.push(v.into());
    }
    pub fn with(mut self, v: impl Value) -> Self {
        self.push(v); self
    }
    pub fn first<T: Value>(&self) -> Option<&T> {
        self.0.iter().find_map(|v| v.downcast_ref::<T>())
    }
    pub fn first_val<T: Value>(&self) -> Option<Val> {
        match self.0.iter().find(|v| v.is::<T>()) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
    pub fn contains<T: Value>(&self) -> bool {
        self.0.iter().any(|v| v.is::<T>())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    v: String,
}
impl Symbol {
    fn to_string(&self) -> String { self.v.clone() }
}
impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str { self.v.as_ref() }
}
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


#[derive(Clone, Eq)]
pub struct Place(Rc<RefCell<Val>>);
impl PartialEq for Place {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl_value!(Place, type_name("place"));

impl Place {
    pub fn wrap(v: impl Value) -> Place {
        Place(Rc::new(RefCell::new(v.into())))
    }
    pub fn swap(&mut self, v: impl Value) -> Val {
        self.0.replace(v.into())
    }
    pub fn set(&mut self, v: impl Value) { self.swap(v); }

    pub fn get(&self) -> Val { self.0.try_borrow().unwrap().clone() }
}

