
use data::*;
use parser::*;
use interpreter::Interpreter;
use interpreter::command::*;
use interpreter::exec;
use stdlib::enumcommand::*;

pub fn install(interpreter: &mut Interpreter) {
    StackOp::install(interpreter);
}

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum StackOp {
    // Stack ops
    Clone, Dig, Drop,

    IsStackEmpty,
}

impl EnumCommand for StackOp {
    fn as_str(&self) -> &str {
        use self::StackOp::*;
        match self {
            Clone => "clone",
            Dig => "dig",
            Drop => "drop",
            IsStackEmpty => "stack-empty?",
        }
    }
    fn last() -> Self { StackOp::IsStackEmpty }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

impl Command for StackOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::StackOp::*;
        match self {
            Clone => {
                let d = { interpreter.stack.ref_datum(0)?.clone() };
                interpreter.stack.push(d);
            },
            Dig => {
                let n = interpreter.stack.pop::<Number>()?.cast::<isize>()?;
                if n > 0 {
                    let n = n as usize;
                    let a = interpreter.stack.remove(n)?;
                    interpreter.stack.push(a);
                } else if n < 0 {
                    let n = -n as usize;
                    let a = interpreter.stack.pop_datum()?;
                    interpreter.stack.insert(a, n - 1)?;
                }
            },
            Drop => {
                interpreter.stack.pop_datum()?;
            },
            IsStackEmpty => {
                let size = interpreter.stack.size();
                interpreter.stack.push(Datum::build().with_source(source).ok(size == 0));
            },
        }
        Ok(())
    }
}


