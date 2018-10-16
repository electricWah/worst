
use stdlib::enumcommand::*;
use interpreter::Interpreter;

mod parser;
mod reader;
mod charclass;

pub fn install(interpreter: &mut Interpreter) {
    self::parser::ParseOp::install(interpreter);
    self::reader::ReaderOp::install(interpreter);
    self::charclass::CharClassOp::install(interpreter);
}


