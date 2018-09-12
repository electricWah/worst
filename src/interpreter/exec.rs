
use std::fmt;
use std::result;
use data::error::Error;

#[derive(Debug)]
pub struct Exception {
    pub error: Box<Error>,
}

impl<T: 'static + Error> From<T> for Exception {
    fn from(error: T) -> Self {
        Exception {
            error: Box::new(error),
        }
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Error: {}", self.error)
    }
}

pub type Result<T> = result::Result<T, Exception>;


