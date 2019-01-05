
use crate::interpreter::Interpreter;

mod parser;
mod reader;
mod charclass;

pub fn install(interpreter: &mut Interpreter) {
    self::parser::install(interpreter);
    self::reader::install(interpreter);
    self::charclass::install(interpreter);
}


