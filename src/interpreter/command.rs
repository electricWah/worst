
use std::fmt;
use crate::parser::Source;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;

pub trait Command: fmt::Debug {
    fn run(&self, interp: &mut Interpreter, src: Option<Source>) -> exec::Result<()>;
}

