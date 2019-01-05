
use crate::stdlib::enumcommand::*;
use crate::interpreter::Interpreter;

pub mod data;
mod ops;

pub use self::data::*;

pub fn install(interpreter: &mut Interpreter) {
    self::ops::U8VectorOp::install(interpreter);
}

