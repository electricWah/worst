
//! Numeric operations that work on i64 and f64 numbers

// Thanks to Racket docs for guidance
// https://docs.racket-lang.org/reference/generic-numbers.html

use super::util;
use crate::base::*;
use crate::interpreter::*;

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

fn unop<T: Value + Clone, R: Value>(f: impl Fn(T) -> R) -> impl Fn(&mut Interpreter) -> BuiltinRet {
    move |i| {
        let a = i.stack_pop::<T>()?;
        i.stack_push(f(a.into_inner()));
        Ok(())
    }
}

/// All binary operations are the same order as infix, so `a b op` infix is `a op b`.
fn binop<T: Value + Clone, R: Value>(f: impl Fn(T, T) -> R) -> impl Fn(&mut Interpreter) -> BuiltinRet {
    move |i| {
        let b = i.stack_pop::<T>()?;
        let a = i.stack_pop::<T>()?;
        i.stack_push(f(a.into_inner(), b.into_inner()));
        Ok(())
    }
}

/// Install numeric functions for i64 and f64
pub fn install(i: &mut Interpreter) {
    util::add_const_type_builtin::<i64>(i, "<i64>");
    util::add_const_type_builtin::<f64>(i, "<f64>");

    i.add_builtin("i64->f64", i64_to_f64);
    i.add_builtin("f64->i64", f64_to_i64);

    i.add_builtin("i64-add", binop::<i64, i64>(|a, b| a + b));
    i.add_builtin("f64-add", binop::<f64, f64>(|a, b| a + b));
    i.add_builtin("i64-sub", binop::<i64, i64>(|a, b| a - b));
    i.add_builtin("f64-sub", binop::<f64, f64>(|a, b| a - b));
    i.add_builtin("i64-mul", binop::<i64, i64>(|a, b| a * b));
    i.add_builtin("f64-mul", binop::<f64, f64>(|a, b| a * b));
    i.add_builtin("i64-div", div_nozero::<i64>);
    i.add_builtin("f64-div", binop::<f64, f64>(|a, b| a / b));

    i.add_builtin("i64-negate", unop::<i64, i64>(|a| -a));
    i.add_builtin("f64-negate", unop::<f64, f64>(|a| -a));

    i.add_builtin("i64-remainder", binop::<i64, i64>(|a, b| a % b));

    i.add_builtin("i64-abs", unop::<i64, i64>(|a| a.abs()));
    i.add_builtin("f64-abs", unop::<f64, f64>(|a| a.abs()));

    i.add_builtin("f64-sqrt", unop::<f64, f64>(|a| a.sqrt()));

    i.add_builtin("f64-sin", unop::<f64, f64>(|a| a.sin()));
    i.add_builtin("f64-cos", unop::<f64, f64>(|a| a.cos()));
    i.add_builtin("f64-tan", unop::<f64, f64>(|a| a.tan()));
    i.add_builtin("f64-asin", unop::<f64, f64>(|a| a.asin()));
    i.add_builtin("f64-acos", unop::<f64, f64>(|a| a.acos()));
    i.add_builtin("f64-atan", unop::<f64, f64>(|a| a.atan()));

    i.add_builtin("i64-bitand", binop::<i64, i64>(|a, b| a & b));
    i.add_builtin("i64-bitxor", binop::<i64, i64>(|a, b| a ^ b));
    i.add_builtin("i64-bitor", binop::<i64, i64>(|a, b| a | b));
    i.add_builtin("i64-bitnot", unop::<i64, i64>(|a| !a));

    i.add_builtin("i64-ashift", binop::<i64, i64>(|a, b| {
        if b > 0 { a << b } else { a >> -b }
    }));
}

