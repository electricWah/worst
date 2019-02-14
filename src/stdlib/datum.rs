
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;
// use crate::interpreter::exec::Failure;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<Symbol>("symbol?");
    interpreter.define_type_predicate::<char>("char?");
    // interpreter.define_type_predicate::<Failure>("failure?");
    interpreter.add_builtin("type-of", type_of);
    interpreter.add_builtin("datum-describe->string", datum_describe_into_string);
    interpreter.add_builtin("describe", describe);
    // interpreter.add_builtin("failure-message", failure_message);
    interpreter.add_builtin("equal?", is_equal);
    interpreter.add_builtin("identical?", is_identical);
}

fn type_of(interpreter: &mut Interpreter) -> exec::Result<()> {
    let s = format!("{}", interpreter.stack.ref_datum(0)?.type_of());
    interpreter.stack.push(Datum::new(s));
    Ok(())
}

fn datum_describe_into_string(interpreter: &mut Interpreter) -> exec::Result<()> {
    let res = {
        let d = interpreter.stack.ref_datum(0)?;
        format!("{}", d.describe())
    };
    interpreter.stack.push(Datum::new(res));
    Ok(())
}

fn describe(interpreter: &mut Interpreter) -> exec::Result<()> {
    let d = interpreter.stack.ref_datum(0)?;
    println!("{}", d.describe());
    Ok(())
}

// fn failure_message(interpreter: &mut Interpreter) -> exec::Result<()> {
//     let msg = interpreter.stack.ref_at::<Failure>(0)?.message();
//     interpreter.stack.push(Datum::new(msg));
//     Ok(())
// }

fn is_equal(interpreter: &mut Interpreter) -> exec::Result<()> {
    let res = {
        let a = interpreter.stack.ref_datum(0)?;
        let b = interpreter.stack.ref_datum(1)?;
        a.value_equal(&b)
    };
    interpreter.stack.push(Datum::new(res));
    Ok(())
}

fn is_identical(interpreter: &mut Interpreter) -> exec::Result<()> {
    let res = {
        let a = interpreter.stack.ref_datum(0)?;
        let b = interpreter.stack.ref_datum(1)?;
        a == b
    };
    interpreter.stack.push(Datum::new(res));
    Ok(())
}

