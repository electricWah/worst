
use std::mem;
use data::*;
use parser::*;
use interpreter::*;
use interpreter::command::*;
use interpreter::exec;
use stdlib::enumcommand::*;

use super::data::*;

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum InterpreterOp {
    CurrentInterpreter,
    MakeInterpreter,

    InterpreterClear,
    InterpreterPushInput,
    InterpreterSwapReader,
    IsInterpreterQuoting,

    IsCurrentInterpreter,
    IsInterpreter,
}

impl EnumCommand for InterpreterOp {
    fn as_str(&self) -> &str {
        use self::InterpreterOp::*;
        match self {
            CurrentInterpreter => "current-interpreter",
            MakeInterpreter => "make-interpreter",
            InterpreterClear => "interpreter-clear",
            InterpreterPushInput => "interpreter-push-input",
            InterpreterSwapReader => "interpreter-swap-reader",
            IsInterpreterQuoting => "interpreter-quoting?",
            IsCurrentInterpreter => "current-interpreter?",
            IsInterpreter => "interpreter?",
        }
    }
    fn last() -> Self { InterpreterOp::IsInterpreter }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

impl Command for InterpreterOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::InterpreterOp::*;
        match self {
            CurrentInterpreter => {
                interpreter.stack.push(Datum::build().with_source(source).ok(InterpRef::current()));
            },
            MakeInterpreter => {
                let reader = interpreter.stack.pop::<Reader>()?;
                let interp = InterpRef::from(Interpreter::new(reader));
                interpreter.stack.push(Datum::build().with_source(source).ok(interp));
            },

            InterpreterClear => {
                if let InterpRef::Ref(ref mut i) = interpreter.stack.top_mut::<InterpRef>()? {
                    return Ok(i.borrow_mut().clear());
                }
                interpreter.clear();
            },
            InterpreterPushInput => {
                let input = interpreter.stack.pop::<String>()?;
                if let InterpRef::Ref(ref mut i) = interpreter.stack.top_mut::<InterpRef>()? {
                    i.borrow_mut().push_input(input.as_str());
                    return Ok(());
                }
                interpreter.push_input(input.as_str());
            },
            InterpreterSwapReader => {
                let (mut reader, src) = interpreter.stack.pop_source::<Reader>()?;
                if let InterpRef::Ref(ref mut i) = interpreter.stack.top_mut::<InterpRef>()? {
                    mem::swap(i.borrow_mut().reader_mut(), &mut reader);
                }
                mem::swap(interpreter.reader_mut(), &mut reader);
                interpreter.stack.push(Datum::build().with_source(src).ok(reader));
            },

            IsInterpreterQuoting => {
                let r = {
                    match interpreter.stack.ref_at::<InterpRef>(0)? {
                        InterpRef::Current => interpreter.quoting(),
                        InterpRef::Ref(ref i) => i.borrow().quoting(),
                    }
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },

            IsCurrentInterpreter => {
                let r = interpreter.stack.ref_at::<InterpRef>(0)? == &InterpRef::Current;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            IsInterpreter => {
                let r = interpreter.stack.type_predicate::<InterpRef>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            // _ => return Err(error::NotImplemented().into()),
        }
        Ok(())
    }
}

