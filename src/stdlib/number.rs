
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<isize>("int?");
    interpreter.define_type_predicate::<f64>("float?");
    interpreter.add_builtin("add", num_add);
    interpreter.add_builtin("negate", num_negate);
    interpreter.add_builtin("mul", num_mul);
    // interpreter.add_builtin("recip", num_recip);
    interpreter.add_builtin("greater-than", num_gt);
    interpreter.add_builtin("abs", num_abs);
    // interpreter.add_builtin("floor", num_floor);
    // interpreter.add_builtin("numerator", num_numerator);
    // interpreter.add_builtin("denominator", num_denominator);
}

fn num_add(interpreter: &mut Interpreter) -> exec::Result<()> {
    let a = interpreter.stack.pop::<isize>()?;
    let b = interpreter.stack.pop::<isize>()?;
    interpreter.stack.push(Datum::new(a + b));
    Ok(())
}

fn num_negate(interpreter: &mut Interpreter) -> exec::Result<()> {
    let a = interpreter.stack.pop::<isize>()?;
    interpreter.stack.push(Datum::new(-a));
    Ok(())
}

fn num_mul(interpreter: &mut Interpreter) -> exec::Result<()> {
    let a = interpreter.stack.pop::<isize>()?;
    let b = interpreter.stack.pop::<isize>()?;
    interpreter.stack.push(Datum::new(a * b));
    Ok(())
}

// fn num_recip(interpreter: &mut Interpreter) -> exec::Result<()> {
//     let a = interpreter.stack.pop::<isize>()?;
//     interpreter.stack.push(Datum::new(a.recip()));
//     Ok(())
// }

fn num_gt(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = {
        let a = interpreter.stack.ref_at::<isize>(0)?;
        let b = interpreter.stack.ref_at::<isize>(1)?;
        a > b
    };
    interpreter.stack.push(Datum::new(r));
    Ok(())
}

fn num_abs(interpreter: &mut Interpreter) -> exec::Result<()> {
    let a = interpreter.stack.pop::<isize>()?;
    interpreter.stack.push(Datum::new(a.abs()));
    Ok(())
}

// fn num_floor(interpreter: &mut Interpreter) -> exec::Result<()> {
//     let a = interpreter.stack.pop::<isize>()?;
//     interpreter.stack.push(Datum::new(a.floor()));
//     Ok(())
// }

// fn num_numerator(interpreter: &mut Interpreter) -> exec::Result<()> {
//     let a = interpreter.stack.ref_at::<isize>(0)?.numerator();
//     interpreter.stack.push(Datum::new(a));
//     Ok(())
// }

// fn num_denominator(interpreter: &mut Interpreter) -> exec::Result<()> {
//     let a = interpreter.stack.ref_at::<isize>(0)?.denominator();
//     interpreter.stack.push(Datum::new(a));
//     Ok(())
// }

