
use std::fmt;
use parser::Source;
use interpreter::Interpreter;
use interpreter::exec;

pub trait Command: fmt::Debug {
    fn run(&self, &mut Interpreter, Option<Source>) -> exec::Result<()>;
}

