
use std::env;
use crate::data::*;
use crate::parser::*;
use crate::interpreter::Interpreter;
use crate::interpreter::command::*;
use crate::interpreter::exec;
use crate::stdlib::enumcommand::*;

use crate::stdlib::hashtable::HashTable;

pub fn install(interpreter: &mut Interpreter) {
    EnvOp::install(interpreter);
}

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum EnvOp {
    CommandLine,
    GetEnvironmentVariable,
    SetEnvironmentVariable,
    GetEnvironmentVariables,
}

impl EnumCommand for EnvOp {
    fn as_str(&self) -> &str {
        use self::EnvOp::*;
        match self {
            CommandLine => "command-line",
            GetEnvironmentVariable => "get-environment-variable",
            SetEnvironmentVariable => "set-environment-variable",
            GetEnvironmentVariables => "get-environment-variables",
        }
    }
    fn last() -> Self { EnvOp::GetEnvironmentVariables }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

impl Command for EnvOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        debug!("EnvOp: {:?}", self);
        use self::EnvOp::*;
        match self {
            CommandLine => {
                let args: Vec<String> = env::args().collect();
                interpreter.stack.push(Datum::build().with_source(source).ok(List::from(args)));
            },

            GetEnvironmentVariable => {
                let var = interpreter.stack.pop::<String>()?;
                let res = env::var(var).ok();
                match res {
                    Some(r) => interpreter.stack.push(Datum::build().with_source(source).ok(r)),
                    None => interpreter.stack.push(Datum::build().with_source(source).ok(false)),
                }
            },
            SetEnvironmentVariable => {
                let v = interpreter.stack.pop::<String>()?;
                let k = interpreter.stack.pop::<String>()?;
                env::set_var(k, v);
            },
            GetEnvironmentVariables => {
                let mut tbl = HashTable::default();
                env::vars().for_each(
                    |(k, v)| tbl.set(Datum::build().ok(k),
                                     Datum::build().ok(v)));
                interpreter.stack.push(Datum::build().with_source(source).ok(tbl));
            },
        }
        Ok(())
    }
}

