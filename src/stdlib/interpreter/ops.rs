
use std::mem;
use data::*;
use parser::*;
use interpreter::*;
use interpreter::command::*;
use interpreter::exec;
use interpreter::code::*;
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
    InterpreterReadNext,
    InterpreterResolveSymbol,
    InterpreterEvalCode,
    InterpreterReadFile,

    InterpreterSwapReader,
    InterpreterSwapStack,

    IsInterpreterQuoting,
    // InterpreterSetQuoting,

    InterpreterIsRootContext,

    InterpreterContextName,
    InterpreterSetContextName,

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
            InterpreterReadNext => "interpreter-read-next",
            InterpreterResolveSymbol => "interpreter-resolve-symbol",
            InterpreterEvalCode => "interpreter-eval-code",
            InterpreterReadFile => "interpreter-read-file",
            InterpreterSwapReader => "interpreter-swap-reader",
            InterpreterSwapStack => "interpreter-swap-stack",
            InterpreterSetContextName => "interpreter-set-context-name",
            InterpreterIsRootContext => "interpreter-root-context?",
            InterpreterContextName => "interpreter-context-name",
            IsInterpreterQuoting => "interpreter-quoting?",
            // InterpreterSetQuoting => "interpreter-set-quoting",
            IsCurrentInterpreter => "current-interpreter?",
            IsInterpreter => "interpreter?",
        }
    }
    fn last() -> Self { InterpreterOp::IsInterpreter }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

fn with_top_mut<T, F: FnOnce(&mut Interpreter) -> T>(i: &mut Interpreter, f: F) -> exec::Result<T> {
    let (mut interp, src) = i.stack.pop_source::<InterpRef>()?;
    let r = match interp {
        InterpRef::Current => f(i),
        InterpRef::Ref(ref mut ii) => f(&mut ii.borrow_mut()),
    };
    i.stack.push(Datum::build().with_source(src).ok(interp));
    Ok(r)
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
                with_top_mut(interpreter, |i| {
                    i.clear();
                })?;
            },
            InterpreterPushInput => {
                let input = interpreter.stack.pop::<String>()?;
                with_top_mut(interpreter, |i| {
                    i.push_input(input.as_str());
                })?;
            },
            InterpreterSwapReader => {
                let (mut reader, src) = interpreter.stack.pop_source::<Reader>()?;
                with_top_mut(interpreter, |i| {
                    mem::swap(i.reader_mut(), &mut reader);
                })?;
                interpreter.stack.push(Datum::build().with_source(src).ok(reader));
            },
            InterpreterSwapStack => {
                let mut l = interpreter.stack.pop::<List>()?.into();
                with_top_mut(interpreter, |i| {
                    mem::swap(i.stack.vec_data_mut(), &mut l);
                })?;
                interpreter.stack.push(Datum::build().with_source(source).ok(List::from(l)));
            },

            &InterpreterIsRootContext => {
                let r = with_top_mut(interpreter, |i| {
                    i.context.is_root()
                })?;
                interpreter.stack.push(Datum::new(r));
            },
            &InterpreterSetContextName => {
                let name = interpreter.stack.pop::<Symbol>()?;
                with_top_mut(interpreter, |i| {
                    i.context.set_name(Some(name.to_string()));
                })?;
            },
            &InterpreterContextName => {
                let name = with_top_mut(interpreter, |i| {
                    i.context.name().map(Symbol::from)
                })?;
                match name {
                    Some(n) => interpreter.stack.push(Datum::new(n)),
                    None => interpreter.stack.push(Datum::new(false)),
                }
            },
            InterpreterReadFile => {
                let file = interpreter.stack.pop::<String>()?;
                let r = with_top_mut(interpreter, |i| {
                    i.load_file(&file)
                })?;
                match r {
                    Ok(()) => {
                        interpreter.stack.push(Datum::new(true));
                    },
                    Err(e) => {
                        interpreter.stack.push(Datum::new(e));
                        interpreter.stack.push(Datum::new(false));
                    },
                }
            },

            InterpreterReadNext => {
                let r = with_top_mut(interpreter, |i| {
                    i.read_next()
                })?;
                match r {
                    Ok(None) => {
                        interpreter.stack.push(Datum::new(false));
                        interpreter.stack.push(Datum::new(true));
                    },
                    Ok(Some(res)) => {
                        interpreter.stack.push(res);
                        interpreter.stack.push(Datum::new(true));
                    },
                    Err(e) => {
                        interpreter.stack.push(Datum::new(e));
                        interpreter.stack.push(Datum::new(false));
                    },
                }
            },

            InterpreterEvalCode => {
                let (code, src) = interpreter.stack.pop_source::<Code>()?;
                let r = with_top_mut(interpreter, |i| {
                    i.eval_code(&code, src)
                })?;
                match r {
                    Ok(()) => {
                        interpreter.stack.push(Datum::new(true));
                    },
                    Err(e) => {
                        interpreter.stack.push(Datum::new(e));
                        interpreter.stack.push(Datum::new(false));
                    },
                }
            },

            InterpreterResolveSymbol => {
                let sym = interpreter.stack.pop::<Symbol>()?;
                let r = with_top_mut(interpreter, |i| {
                    i.resolve_symbol(&sym)
                })?;
                match r {
                    Some(r) => interpreter.stack.push(Datum::new(r)),
                    None => interpreter.stack.push(Datum::new(false)),
                }
            },

            IsInterpreterQuoting => {
                let r = with_top_mut(interpreter, |i| {
                    i.quoting()
                })?;
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

