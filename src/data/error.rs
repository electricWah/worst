
use std::io;
use std::rc::Rc;
use crate::data::*;
use crate::data::value::*;
use crate::data::types::*;

pub trait BuiltinError: Clone {
    fn name(&self) -> &'static str;
    fn args(&self) -> Vec<Datum> { vec![] }
}

#[derive(Debug, Clone)]
pub struct Abort;
impl BuiltinError for Abort { fn name(&self) -> &'static str { "abort" } }

#[derive(Debug, Clone)]
pub struct NotDefined(pub Symbol);
impl BuiltinError for NotDefined {
    fn name(&self) -> &'static str { "not-defined" }
    fn args(&self) -> Vec<Datum> {
        vec![Datum::new(self.0.clone())]
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct StackEmpty;
impl BuiltinError for StackEmpty { fn name(&self) -> &'static str { "stack-empty" } }

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct WrongType(pub Type, pub Type);
impl BuiltinError for WrongType {
    fn name(&self) -> &'static str { "wrong-type" }
    fn args(&self) -> Vec<Datum> {
        vec![Datum::symbol(self.0.as_str()), Datum::symbol(self.1.as_str())]
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

impl ValueHash for StdIoError {}

impl BuiltinError for StdIoError {
    fn name(&self) -> &'static str { "io-error" }
    fn args(&self) -> Vec<Datum> {
        vec![Datum::new(format!("{:?}", self.0.kind()))]
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct UplevelInRootContext;
impl BuiltinError for UplevelInRootContext { fn name(&self) -> &'static str { "root-uplevel" } }

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct WrongSize(pub isize, pub isize);
impl BuiltinError for WrongSize {
    fn name(&self) -> &'static str { "wrong-size" }
    fn args(&self) -> Vec<Datum> {
        vec![Datum::new(self.0.clone()), Datum::new(self.1.clone())]
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct OutOfRange(pub isize, pub isize, pub isize);
impl BuiltinError for OutOfRange {
    fn name(&self) -> &'static str { "out-of-range" }
    fn args(&self) -> Vec<Datum> {
        vec![Datum::new(self.0.clone()), Datum::new(self.1.clone()), Datum::new(self.2.clone())]
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct ConversionFailure;
impl BuiltinError for ConversionFailure { fn name(&self) -> &'static str { "conversion-failure" } }

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct ListEmpty;
impl BuiltinError for ListEmpty { fn name(&self) -> &'static str { "list-empty" } }

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct NotUnique;
impl BuiltinError for NotUnique { fn name(&self) -> &'static str { "not-unique" } }

