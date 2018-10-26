
use std::rc::Rc;
use std::fmt;
use std::result;
use data::*;
use data::error::Error;

#[derive(Debug, Clone)]
pub struct Failure {
    pub error: Rc<Box<Error>>,
}

impl<T: 'static + Error> From<T> for Failure {
    fn from(error: T) -> Self {
        Failure {
            error: Rc::new(Box::new(error)),
        }
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Error: {}", self.error)
    }
}

impl StaticType for Failure {
    fn static_type() -> Type {
        Type::new("failure")
    }
}

impl DefaultValueClone for Failure {}
impl ValueDebugDescribe for Failure {}
impl ValueShow for Failure {}
impl ValueEq for Failure {}
impl ValueHash for Failure {}
impl Value for Failure {}

pub type Result<T> = result::Result<T, Failure>;


