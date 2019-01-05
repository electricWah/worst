
use crate::data::*;
use crate::parser::*;
use crate::interpreter::Interpreter;
use crate::interpreter::command::*;
use crate::interpreter::exec;
use crate::stdlib::enumcommand::*;

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BoolOp {
    And, Or,
    IsBool,
}

impl EnumCommand for BoolOp {
    fn as_str(&self) -> &str {
        use self::BoolOp::*;
        match self {
            And => "and",
            Or => "or",
            IsBool => "bool?",
        }
    }
    fn last() -> Self { BoolOp::IsBool }
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
            IsBool => {
                let r = interpreter.stack.type_predicate::<bool>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
        }
        Ok(())
    }
}


