
use stdlib::enumcommand::*;
use interpreter::Interpreter;

pub mod data;
mod ops;

pub use data::*;

pub fn install(interpreter: &mut Interpreter) {
    self::ops::U8VectorOp::install(interpreter);
}

