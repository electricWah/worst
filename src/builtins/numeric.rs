
//! Numeric operations that work on i64 and f64 numbers

// Thanks to Racket docs for guidance
// https://docs.racket-lang.org/reference/generic-numbers.html

use super::util;
use crate::base::*;
use crate::interpreter::*;

// TODO pop two at once with stack_pop::<(T, T)>

/// Add the top two numbers on the stack.
pub fn add<T: std::ops::Add<T, Output=T> + Value + Clone>(i: &mut Interpreter) -> BuiltinRet {
    let b = i.stack_pop::<T>()?;
    let a = i.stack_pop::<T>()?;
    i.stack_push(a.into_inner() + b.into_inner());
    Ok(())
}

/// `a b sub` is `a - b`.
/// If you need to remember which way around it is, `sub` is `negate add`.
/// All of these operations are the same order as infix, so
/// `a b op` infix is `a op b`.
pub fn sub<T: std::ops::Sub<T, Output=T> + Value + Clone>(i: &mut Interpreter) -> BuiltinRet {
    let b = i.stack_pop::<T>()?;
    let a = i.stack_pop::<T>()?;
    i.stack_push(a.into_inner() - b.into_inner());
    Ok(())
}

/// `a b mul` is `a * b`.
pub fn mul<T: std::ops::Mul<T, Output=T> + Value + Clone>(i: &mut Interpreter) -> BuiltinRet {
    let b = i.stack_pop::<T>()?;
    let a = i.stack_pop::<T>()?;
    i.stack_push(a.into_inner() * b.into_inner());
    Ok(())
}

/// `a b div` is `a / b`.
/// Integer division rounds towards zero.
/// If you need to remember which way around it is, `div` is `recip mul` for f64s.
/// All of these operations are the same order as infix, so
/// `a b op` infix is `a op b`.
pub fn div<T: std::ops::Div<T, Output=T> + Value + Clone>(i: &mut Interpreter) -> BuiltinRet {
    let b = i.stack_pop::<T>()?;
    let a = i.stack_pop::<T>()?;
    i.stack_push(a.into_inner() / b.into_inner());
    Ok(())
}

/// Division that produces a `false` `error?`
/// instead of crashing when dividing by 0.
/// See [div] for more information.
pub fn div_nozero<T: std::ops::Div<T, Output=T> + Value + Clone + std::cmp::PartialEq<T> + From<i8>>(i: &mut Interpreter) -> BuiltinRet {
    let b = i.stack_pop::<T>()?.into_inner();
    let a = i.stack_pop::<T>()?.into_inner();
    if b == 0.into() {
        i.stack_push_error(false);
    } else {
        i.stack_push(a / b);
    }
    Ok(())
}

/// Negate the number on top of the stack.
pub fn negate<T: std::ops::Neg<Output=T> + Value + Clone>(i: &mut Interpreter) -> BuiltinRet {
    let a = i.stack_pop::<T>()?;
    i.stack_push(-a.into_inner());
    Ok(())
}

/// Calculate the absolute value of the i64 on top of the stack.
pub fn abs_i64(i: &mut Interpreter) -> BuiltinRet {
    let a = i.stack_pop::<i64>()?;
    i.stack_push(a.into_inner().abs());
    Ok(())
}

/// Calculate the absolute value of the f64 on top of the stack.
pub fn abs_f64(i: &mut Interpreter) -> BuiltinRet {
    let a = i.stack_pop::<f64>()?;
    i.stack_push(a.into_inner().abs());
    Ok(())
}

/// Convert the [i64] on top of the stack to [f64].
pub fn i64_to_f64(i: &mut Interpreter) -> BuiltinRet {
    let a = i.stack_pop::<i64>()?;
    i.stack_push(a.into_inner() as f64);
    Ok(())
}

/// Convert the [f64] on top of the stack to [i64].
pub fn f64_to_i64(i: &mut Interpreter) -> BuiltinRet {
    let a = i.stack_pop::<f64>()?;
    i.stack_push(a.into_inner() as i64);
    Ok(())
}

/// Install numeric functions for i64 and f64
pub fn install(i: &mut Interpreter) {
    util::add_type_predicate_builtin::<i64>(i, "i64?");
    util::add_type_predicate_builtin::<f64>(i, "f64?");

    i.add_builtin("i64->string", util::value_tostring_debug::<i64>);
    i.add_builtin("f64->string", util::value_tostring_debug::<f64>);

    i.add_builtin("i64-equal", util::equality::<i64>);
    i.add_builtin("f64-equal", util::equality::<f64>);
    i.add_builtin("i64-compare", util::comparison::<i64>);
    i.add_builtin("f64-compare", util::comparison::<f64>);
    i.add_builtin("i64-hash", util::value_hash::<i64>);
    // f64 not hashable
    // i.add_builtin("f64-hash", util::value_hash::<f64>);

    i.add_builtin("i64-add", add::<i64>);
    i.add_builtin("f64-add", add::<f64>);
    i.add_builtin("i64-sub", sub::<i64>);
    i.add_builtin("f64-sub", sub::<f64>);
    i.add_builtin("i64-mul", mul::<i64>);
    i.add_builtin("f64-mul", mul::<f64>);
    i.add_builtin("i64-div", div_nozero::<i64>);
    i.add_builtin("f64-div", div::<f64>);

    i.add_builtin("i64-negate", negate::<i64>);
    i.add_builtin("f64-negate", negate::<f64>);

    // quotient
    // remainder
    // quotient/  remainder
    // modulo

    i.add_builtin("i64-abs", abs_i64);
    i.add_builtin("f64-abs", abs_f64);

    i.add_builtin("i64->f64", i64_to_f64);
    i.add_builtin("f64->i64", f64_to_i64);
}

