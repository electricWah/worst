
use std::result;
use crate::data::*;
use crate::data::error::BuiltinError;

#[derive(Debug, Clone)]
pub struct Failure {
    name: String,
    args: Vec<Datum>,
}

impl<T: 'static + BuiltinError> From<T> for Failure {
    fn from(error: T) -> Self {
        Failure {
            name: error.name().to_string(),
            args: error.args(),
        }
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

