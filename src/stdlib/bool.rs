
use data::*;
use parser::*;
use interpreter::Interpreter;
use interpreter::command::*;
use interpreter::exec;
use stdlib::enumcommand::*;

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BoolOp {
    And, Or, Not,
}

impl EnumCommand for BoolOp {
    fn as_str(&self) -> &str {
        use self::BoolOp::*;
        match self {
            And => "and",
            Or => "or",
            Not => "not",
        }
    }
    fn last() -> Self { BoolOp::Not }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

pub fn install(interpreter: &mut Interpreter) {
    BoolOp::install(interpreter);
}

impl Command for BoolOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::BoolOp::*;
        match self {
            And => {
                // interpreter.stack.expect(&[DatumType::Boolean.into(), DatumType::Boolean.into()])?;
                let res = {
                    let a = interpreter.stack.ref_datum(0)?;
                    let b = interpreter.stack.ref_datum(1)?;
                    !(a.value_ref::<bool>() == Ok(&false) || b.value_ref::<bool>() == Ok(&false))
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(res));
            },
            Or => {
                let res = {
                    let a = interpreter.stack.ref_datum(0)?;
                    let b = interpreter.stack.ref_datum(1)?;
                    a.value_ref::<bool>() != Ok(&false) || b.value_ref::<bool>() != Ok(&false)
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(res));
            },
            Not => {
                // interpreter.stack.expect(&[DatumType::Boolean.into()])?;
                let res = {
                    let a = interpreter.stack.ref_datum(0)?;
                    a.value_ref::<bool>() == Ok(&false)
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(res));
            },
        }
        Ok(())
    }
}


