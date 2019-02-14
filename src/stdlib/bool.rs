
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<bool>("bool?");
    interpreter.add_builtin("and", bool_and);
    interpreter.add_builtin("or", bool_or);
}

fn bool_and(interpreter: &mut Interpreter) -> exec::Result<()> {
    let res = {
        let a = interpreter.stack.ref_datum(0)?;
        let b = interpreter.stack.ref_datum(1)?;
        !(a.value_ref::<bool>() == Ok(&false) || b.value_ref::<bool>() == Ok(&false))
    };
    interpreter.stack.push(Datum::new(res));
    Ok(())
}

fn bool_or(interpreter: &mut Interpreter) -> exec::Result<()> {
    let res = {
        let a = interpreter.stack.ref_datum(0)?;
        let b = interpreter.stack.ref_datum(1)?;
        a.value_ref::<bool>() != Ok(&false) || b.value_ref::<bool>() != Ok(&false)
    };
    interpreter.stack.push(Datum::new(res));
    Ok(())
}


