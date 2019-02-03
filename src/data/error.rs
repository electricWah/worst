
pub use std::error::Error;
use std::fmt;
use std::io;
use std::rc::Rc;
use crate::data::Symbol;
use crate::data::value::*;
use crate::data::types::*;

#[derive(Debug)]
pub struct Abort();
impl Error for Abort {}
impl fmt::Display for Abort {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Execution aborted")
    }
}

#[derive(Debug)]
pub struct NotDefined(pub Symbol);
impl Error for NotDefined {}

impl fmt::Display for NotDefined {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Not defined: '{}'", self.0.as_ref())
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct StackEmpty();
impl Error for StackEmpty {}

impl fmt::Display for StackEmpty {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Stack empty")
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct WrongType(pub Type, pub Type);
impl Error for WrongType {}

impl fmt::Display for WrongType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Wrong type: expected {}, but got {}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
pub struct StdIoError(Rc<io::Error>);

impl StdIoError {
    pub fn new(err: io::Error) -> Self {
        StdIoError(Rc::new(err))
    }
}

impl PartialEq for StdIoError {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
impl Eq for StdIoError {}

impl Error for StdIoError {}
impl ValueHash for StdIoError {}

impl fmt::Display for StdIoError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "IO error: {}", self.0)
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct UplevelInRootContext();
impl Error for UplevelInRootContext {}

impl fmt::Display for UplevelInRootContext {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Uplevel in root context")
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct WrongSize(pub isize, pub isize);
impl Error for WrongSize {}

impl fmt::Display for WrongSize {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Wrong size: expected {}, but got {}", self.0, self.1)
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct OutOfRange(pub isize, pub isize, pub isize);
impl Error for OutOfRange {}

impl fmt::Display for OutOfRange {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Out of range: expected {} - {}, but got {}", self.0, self.1, self.2)
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct ConversionFailure();
impl Error for ConversionFailure {}

impl fmt::Display for ConversionFailure {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Conversion failure")
    }
}


#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct ListEmpty();
impl Error for ListEmpty {}

impl fmt::Display for ListEmpty {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "List empty")
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct NotUnique();
impl Error for NotUnique {}

impl fmt::Display for NotUnique {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Value is not unique; another reference to it also exists")
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct NotImplemented();
impl Error for NotImplemented {}

impl fmt::Display for NotImplemented {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Not implemented")
    }
}


