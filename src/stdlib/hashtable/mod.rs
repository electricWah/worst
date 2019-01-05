
use crate::stdlib::enumcommand::*;
use crate::interpreter::Interpreter;
mod data;
mod ops;

pub use self::data::HashTable;

pub fn install(interpreter: &mut Interpreter) {
    self::ops::HashTableOp::install(interpreter);
}


