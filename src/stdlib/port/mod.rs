
use crate::stdlib::enumcommand::*;
use crate::interpreter::Interpreter;

mod data;
mod port;
mod ops;

mod stdio;

pub use self::data::Port;
pub use self::port::IsPort;

pub fn install(interpreter: &mut Interpreter) {
    self::ops::IoOp::install(interpreter);
}
