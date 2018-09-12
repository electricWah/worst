
use std::str::FromStr;
use data::error;
use interpreter::Interpreter;
use interpreter::command::Command;

pub trait EnumCommand: Eq {
    fn as_str(&self) -> &str;
    fn last() -> Self;
    fn from_usize(usize) -> Self;
}

pub trait InterpreterCommand {
    fn install(&mut Interpreter);
}

impl<T: 'static + EnumCommand + Command + Clone> InterpreterCommand for T {
    fn install(interpreter: &mut Interpreter) {
        let mut cmd_u = 0;
        let last = T::last();
        loop {
            let cmd = T::from_usize(cmd_u);
            interpreter.define(cmd.as_str(), cmd.clone());
            if cmd == last {
                break;
            } else {
                cmd_u = cmd_u + 1;
            }
        }
    }
}

struct EnumCommandFromStr<T>(T);

impl<T: EnumCommand> FromStr for EnumCommandFromStr<T> {
    type Err = error::NotDefined;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cmd_u = 0;
        let last = T::last();
        loop {
            let cmd = T::from_usize(cmd_u);
            if cmd.as_str() == s {
                return Ok(EnumCommandFromStr(cmd));
            }
            if cmd == last {
                break;
            } else {
                cmd_u = cmd_u + 1;
            }
        }
        return Err(error::NotDefined())
    }
}

