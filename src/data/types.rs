
use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Type(String);

impl Type {
    pub fn new<S: Into<String>>(s: S) -> Type {
        Type(s.into())
    }
}

pub trait IsType {
    fn get_type() -> Type;
}

pub trait HasType {
    fn type_of(&self) -> Type;
}

pub trait StaticType {
    fn static_type() -> Type;
}

impl<T: StaticType> IsType for T {
    fn get_type() -> Type {
        T::static_type()
    }
}

impl<T: StaticType> HasType for T {
    fn type_of(&self) -> Type {
        T::static_type()
    }
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}


