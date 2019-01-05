
use crate::stdlib::enumcommand::*;
use crate::interpreter::Interpreter;

mod data;
mod ops;

pub fn install(interpreter: &mut Interpreter) {
    self::ops::InterpreterOp::install(interpreter);
}


