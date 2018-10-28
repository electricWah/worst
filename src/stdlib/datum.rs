
use data::*;
use parser::*;
use interpreter::Interpreter;
use interpreter::command::*;
use interpreter::exec;
use interpreter::exec::Failure;
use stdlib::enumcommand::*;

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DatumOp {
    // Meta
    TypeOf,
    HasSource,

    IsSymbol,
    IsChar,

    DatumDescribeToString,

    IsFailure,
    FailureMessage,

    // Comparison
    Equal, // Inner values are equal
    Identical, // Datums are fully equal
    // Gt, Gte,
}

impl EnumCommand for DatumOp {
    fn as_str(&self) -> &str {
        use self::DatumOp::*;
        match self {
            TypeOf => "type-of",
            HasSource => "has-source?",
            IsSymbol => "symbol?",
            IsChar => "char?",
            DatumDescribeToString => "datum-describe->string",
            IsFailure => "failure?",
            FailureMessage => "failure-message",
            Equal => "equal?",
            Identical => "identical?",
        }
    }
    fn last() -> Self { DatumOp::Identical }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

pub fn install(interpreter: &mut Interpreter) {
    DatumOp::install(interpreter)
}

impl Command for DatumOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::DatumOp::*;
        match self {
            TypeOf => {
                let s = format!("{}", interpreter.stack.ref_datum(0)?.type_of());
                interpreter.stack.push(Datum::build().with_source(source).symbol(s));
            },

            HasSource => {
                let res = {
                    let d = interpreter.stack.ref_datum(0)?;
                    d.source().is_some()
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(res));
            },

            IsSymbol => {
                let r = interpreter.stack.type_predicate::<Symbol>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },

            IsChar => {
                let r = interpreter.stack.type_predicate::<char>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },

            DatumDescribeToString => {
                let res = {
                    let d = interpreter.stack.ref_datum(0)?;
                    format!("{}", d.describe())
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(res));
            },

            IsFailure => {
                let r = interpreter.stack.type_predicate::<Failure>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },

            FailureMessage => {
                let msg = interpreter.stack.ref_at::<Failure>(0)?.message();
                interpreter.stack.push(Datum::build().with_source(source).ok(msg));
            },

            Equal => {
                let res = {
                    let a = interpreter.stack.ref_datum(0)?;
                    let b = interpreter.stack.ref_datum(1)?;
                    a.value_equal(&b)
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(res));
            },
            Identical => {
                let res = {
                    let a = interpreter.stack.ref_datum(0)?;
                    let b = interpreter.stack.ref_datum(1)?;
                    a == b
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(res));
            },

        }
        Ok(())
    }
}


