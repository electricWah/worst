
use crate::data::*;
use crate::parser::*;
use crate::interpreter::Interpreter;
use crate::interpreter::command::*;
use crate::interpreter::exec;
use crate::stdlib::enumcommand::*;

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum NumericOp {
    // Numeric
    Add, Negate,
    Mul, Recip,
    // DivMod,
    GreaterThan,

    Abs,
    Floor,

    Numerator,
    Denominator,

    IsNumber,
}

impl EnumCommand for NumericOp {
    fn as_str(&self) -> &str {
        use self::NumericOp::*;
        match self {
            Add => "add",
            Negate => "negate",
            Mul => "mul",
            Recip => "recip",
            GreaterThan => "greater-than",
            Abs => "abs",
            Floor => "floor",
            Numerator => "numerator",
            Denominator => "denominator",
            IsNumber => "number?",
        }
    }
    fn last() -> Self { NumericOp::IsNumber }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

pub fn install(interpreter: &mut Interpreter) {
    NumericOp::install(interpreter)
}

impl Command for NumericOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::NumericOp::*;
        match self {
            Add => {
                let a = interpreter.stack.pop::<Number>()?;
                let b = interpreter.stack.pop::<Number>()?;
                interpreter.stack.push(Datum::build().with_source(source).ok(a + b));
            },
            Negate => {
                let a = interpreter.stack.pop::<Number>()?;
                interpreter.stack.push(Datum::build().with_source(source).ok(-a));
            },
            Mul => {
                let a = interpreter.stack.pop::<Number>()?;
                let b = interpreter.stack.pop::<Number>()?;
                interpreter.stack.push(Datum::build().with_source(source).ok(a * b));
            },
            Recip => {
                let a = interpreter.stack.pop::<Number>()?;
                interpreter.stack.push(Datum::build().with_source(source).ok(a.recip()));
            },
            GreaterThan => {
                let r = {
                    let a = interpreter.stack.ref_at::<Number>(0)?;
                    let b = interpreter.stack.ref_at::<Number>(1)?;
                    a > b
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            Abs => {
                let a = interpreter.stack.pop::<Number>()?;
                interpreter.stack.push(Datum::build().with_source(source).ok(a.abs()));
            },
            Floor => {
                let a = interpreter.stack.pop::<Number>()?;
                interpreter.stack.push(Datum::build().with_source(source).ok(a.floor()));
            },
            Numerator => {
                let a = interpreter.stack.ref_at::<Number>(0)?.numerator();
                interpreter.stack.push(Datum::build().with_source(source).ok(a));
            },
            Denominator => {
                let a = interpreter.stack.ref_at::<Number>(0)?.denominator();
                interpreter.stack.push(Datum::build().with_source(source).ok(a));
            },
            IsNumber => {
                let v = interpreter.stack.type_predicate::<Number>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(v));
            },
        }
        Ok(())
    }
}

