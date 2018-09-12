
use std::str::FromStr;
use std::fmt;
use std::ops;
use std::cmp;
use num_rational::BigRational;
use num_traits::{ToPrimitive, Signed};
use data::*;
use data::error::*;
use interpreter::exec;

pub type Exact = BigRational;
// pub type Inexact = f64;

pub enum NumRef<'a> {
    Exact(&'a Exact),
    // Inexact(&'a Inexact),
}

// pub trait Numeric {
//     fn is_exact(&self) -> bool;

//     fn num_ref(&self) -> NumRef;
//     fn to_exact(self) -> Exact;
// }
// downcast!(Numeric);

#[derive(Clone)]
pub struct Number(Exact);

impl FromStr for Number {
    type Err = <BigRational as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Exact::from_str(s).map(Number::exact)
    }
}

impl IsType for Number {
    fn get_type() -> Type {
        Type::new("number")
    }
}

impl HasType for Number {
    fn type_of(&self) -> Type {
        if self.0.is_integer() {
            Type::new("integer")
        } else {
            Type::new("number")
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Number {}

impl fmt::Display for Number {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl ValueHash for Number {
}

impl DefaultValueEq for Number {}
impl DefaultValueClone for Number {}
impl ValueDebugDescribe for Number {}
impl ValueDisplayShow for Number {}

impl Value for Number {}

impl ops::Add for Number {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Number::exact(self.0 + other.0)
    }
}

impl ops::Neg for Number {
    type Output = Self;
    fn neg(self) -> Self {
        Number::exact(-self.0)
    }
}

impl ops::Mul for Number {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Number::exact(self.0 * other.0)
    }
}

impl cmp::PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

// impl Numeric for Exact {
//     fn is_exact(&self) -> bool { true }
//     fn to_exact(self) -> Exact { self }
//     fn num_ref(&self) -> NumRef { NumRef::Exact(&self) }
// }

// impl Numeric for Inexact {
//     fn is_exact(&self) -> bool { false }
//     fn num_ref(&self) -> NumRef { NumRef::Inexact(&self) }
// }

#[derive(Debug)]
pub enum NumberConvertError {
    // Negative number to unsigned, or negate an unsigned machine int
    Sign,
    // Too big to fit
    Range,
    // e.g. 0.5 to isize
    Precision,
    // Inexact to exact
    // Exactness,
}

impl Error for NumberConvertError {}
impl fmt::Display for NumberConvertError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // TODO nicer
        match self {
            &NumberConvertError::Sign => write!(fmt, "Sign error"),
            &NumberConvertError::Range => write!(fmt, "Range error"),
            &NumberConvertError::Precision => write!(fmt, "Precision error"),
            // &NumberConvertError::Exactness => write!(fmt, "Exactness error"),
        }
    }
}

pub trait FromNumber: Sized {
    fn from_exact(&Exact) -> Result<Self, NumberConvertError>;
    // fn from_inexact(&Inexact) -> Result<Self, NumberConvertError>;
}

pub trait IntoExact {
    fn into_exact(self) -> Exact;
}
pub trait DefaultIntoExact {}

impl<T: Into<Exact> + DefaultIntoExact> IntoExact for T {
    fn into_exact(self) -> Exact { self.into() }
}

impl DefaultIntoExact for Exact {}

impl Number {
    // pub fn num_ref(&self) -> NumRef {
    //     Numeric::num_ref(&*self.0)
    // }

    // pub fn is_exact(&self) -> bool {
    //     self.0.is_exact()
    // }
    // pub fn to_exact(self) -> Exact {
    //     self.0.0
    // }

    pub fn exact<T: IntoExact>(t: T) -> Self {
        Number(t.into_exact())
    }
    // pub fn inexact<T: Into<Inexact>>(t: T) -> Self {
    //     Number(Box::new(t.into()))
    // }
    // Ref cast; got rid of move cast; is that ok?
    pub fn cast<T: FromNumber>(&self) -> exec::Result<T> {
        T::from_exact(&self.0).map_err(Into::into)
    }

    pub fn is_integer(&self) -> bool {
        self.0.is_integer()
    }

    pub fn recip(self) -> Self {
        Number::exact(self.0.recip())
    }
    pub fn abs(self) -> Self {
        Number::exact(self.0.abs())
    }
    pub fn floor(self) -> Self {
        Number::exact(self.0.floor())
    }

}

impl IntoExact for usize {
    fn into_exact(self) -> Exact {
        Exact::from_integer(self.into())
    }
}

impl FromNumber for usize {
    fn from_exact(n: &Exact) -> Result<Self, NumberConvertError> {
        if n.is_integer() {
            n.to_integer().to_usize().ok_or(NumberConvertError::Range)
        } else {
            Err(NumberConvertError::Precision)
        }
    }
}

impl FromNumber for isize {
    fn from_exact(n: &Exact) -> Result<Self, NumberConvertError> {
        if n.is_integer() {
            n.to_integer().to_isize().ok_or(NumberConvertError::Range)
        } else {
            Err(NumberConvertError::Precision)
        }
    }
}

impl IntoExact for u8 {
    fn into_exact(self) -> Exact {
        Exact::from_integer(self.into())
    }
}

impl FromNumber for u8 {
    fn from_exact(n: &Exact) -> Result<Self, NumberConvertError> {
        if n.is_integer() {
            n.to_integer().to_u8().ok_or(NumberConvertError::Range)
        } else {
            Err(NumberConvertError::Precision)
        }
    }
}

impl IntoExact for i8 {
    fn into_exact(self) -> Exact {
        Exact::from_integer(self.into())
    }
}

impl FromNumber for i8 {
    fn from_exact(n: &Exact) -> Result<Self, NumberConvertError> {
        if n.is_integer() {
            n.to_integer().to_i8().ok_or(NumberConvertError::Range)
        } else {
            Err(NumberConvertError::Precision)
        }
    }
}

impl IntoExact for u16 {
    fn into_exact(self) -> Exact {
        Exact::from_integer(self.into())
    }
}

impl FromNumber for u16 {
    fn from_exact(n: &Exact) -> Result<Self, NumberConvertError> {
        if n.is_integer() {
            n.to_integer().to_u16().ok_or(NumberConvertError::Range)
        } else {
            Err(NumberConvertError::Precision)
        }
    }
}

impl IntoExact for i16 {
    fn into_exact(self) -> Exact {
        Exact::from_integer(self.into())
    }
}

impl FromNumber for i16 {
    fn from_exact(n: &Exact) -> Result<Self, NumberConvertError> {
        if n.is_integer() {
            n.to_integer().to_i16().ok_or(NumberConvertError::Range)
        } else {
            Err(NumberConvertError::Precision)
        }
    }
}

impl IntoExact for u32 {
    fn into_exact(self) -> Exact {
        Exact::from_integer(self.into())
    }
}

impl FromNumber for u32 {
    fn from_exact(n: &Exact) -> Result<Self, NumberConvertError> {
        if n.is_integer() {
            n.to_integer().to_u32().ok_or(NumberConvertError::Range)
        } else {
            Err(NumberConvertError::Precision)
        }
    }
}

impl IntoExact for i32 {
    fn into_exact(self) -> Exact {
        Exact::from_integer(self.into())
    }
}

impl FromNumber for i32 {
    fn from_exact(n: &Exact) -> Result<Self, NumberConvertError> {
        if n.is_integer() {
            n.to_integer().to_i32().ok_or(NumberConvertError::Range)
        } else {
            Err(NumberConvertError::Precision)
        }
    }
}

impl IntoExact for u64 {
    fn into_exact(self) -> Exact {
        Exact::from_integer(self.into())
    }
}

impl FromNumber for u64 {
    fn from_exact(n: &Exact) -> Result<Self, NumberConvertError> {
        if n.is_integer() {
            n.to_integer().to_u64().ok_or(NumberConvertError::Range)
        } else {
            Err(NumberConvertError::Precision)
        }
    }
}

impl IntoExact for i64 {
    fn into_exact(self) -> Exact {
        Exact::from_integer(self.into())
    }
}

impl FromNumber for i64 {
    fn from_exact(n: &Exact) -> Result<Self, NumberConvertError> {
        if n.is_integer() {
            n.to_integer().to_i64().ok_or(NumberConvertError::Range)
        } else {
            Err(NumberConvertError::Precision)
        }
    }
}

