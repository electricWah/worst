
//! Numeric operations that work on i64 and f64 numbers

// Thanks to Racket docs for guidance
// https://docs.racket-lang.org/reference/generic-numbers.html

use super::util;
use crate::base::*;
use crate::interpreter::{Interpreter, Handle};

// TODO pop two at once with stack_pop::<(T, T)>

/// Add the top two numbers on the stack.
pub async fn add<T: std::ops::Add<T, Output=T> + ImplValue + Clone + 'static>(mut i: Handle) {
    let b = i.stack_pop::<T>().await;
    let a = i.stack_pop::<T>().await;
    i.stack_push(a.into_inner() + b.into_inner()).await;
}

/// `a b sub` is `a - b`.
/// If you need to remember which way around it is, `sub` is `negate add`.
/// All of these operations are the same order as infix, so
/// `a b op` infix is `a op b`.
pub async fn sub<T: std::ops::Sub<T, Output=T> + ImplValue + Clone + 'static>(mut i: Handle) {
    let b = i.stack_pop::<T>().await;
    let a = i.stack_pop::<T>().await;
    i.stack_push(a.into_inner() - b.into_inner()).await;
}

/// `a b mul` is `a * b`.
pub async fn mul<T: std::ops::Mul<T, Output=T> + ImplValue + Clone + 'static>(mut i: Handle) {
    let b = i.stack_pop::<T>().await;
    let a = i.stack_pop::<T>().await;
    i.stack_push(a.into_inner() * b.into_inner()).await;
}

/// `a b div` is `a / b`.
/// Integer division rounds towards zero.
/// If you need to remember which way around it is, `div` is `recip mul` for f64s.
/// All of these operations are the same order as infix, so
/// `a b op` infix is `a op b`.
pub async fn div<T: std::ops::Div<T, Output=T> + ImplValue + Clone + 'static>(mut i: Handle) {
    let b = i.stack_pop::<T>().await;
    let a = i.stack_pop::<T>().await;
    i.stack_push(a.into_inner() / b.into_inner()).await;
}

/// Negate the number on top of the stack.
pub async fn negate<T: std::ops::Neg<Output=T> + ImplValue + Clone + 'static>(mut i: Handle) {
    let a = i.stack_pop::<T>().await;
    i.stack_push(-a.into_inner()).await;
}

/// Calculate the absolute value of the i64 on top of the stack.
pub async fn abs_i64(mut i: Handle) {
    let a = i.stack_pop::<i64>().await;
    i.stack_push(a.into_inner().abs()).await;
}

/// Calculate the absolute value of the f64 on top of the stack.
pub async fn abs_f64(mut i: Handle) {
    let a = i.stack_pop::<f64>().await;
    i.stack_push(a.into_inner().abs()).await;
}

/// `a b lt` is `a < b`
pub async fn lt<T: std::cmp::PartialOrd<T> + ImplValue + Clone + 'static>(mut i: Handle) {
    let b = i.stack_pop::<T>().await;
    let a = i.stack_pop::<T>().await;
    i.stack_push(a.into_inner() < b.into_inner()).await;
}
/// `a b le` is `a <= b`
pub async fn le<T: std::cmp::PartialOrd<T> + ImplValue + Clone + 'static>(mut i: Handle) {
    let b = i.stack_pop::<T>().await;
    let a = i.stack_pop::<T>().await;
    i.stack_push(a.into_inner() <= b.into_inner()).await;
}

/// `a b gt` is `a > b`
pub async fn gt<T: std::cmp::PartialOrd<T> + ImplValue + Clone + 'static>(mut i: Handle) {
    let b = i.stack_pop::<T>().await;
    let a = i.stack_pop::<T>().await;
    i.stack_push(a.into_inner() > b.into_inner()).await;
}

/// `a b ge` is `a >= b`
pub async fn ge<T: std::cmp::PartialOrd<T> + ImplValue + Clone + 'static>(mut i: Handle) {
    let b = i.stack_pop::<T>().await;
    let a = i.stack_pop::<T>().await;
    i.stack_push(a.into_inner() >= b.into_inner()).await;
}

/// Convert the [i64] on top of the stack to [f64].
pub async fn i64_to_f64(mut i: Handle) {
    let a = i.stack_pop::<i64>().await;
    i.stack_push(a.into_inner() as f64).await;
}

/// Convert the [f64] on top of the stack to [i64].
pub async fn f64_to_i64(mut i: Handle) {
    let a = i.stack_pop::<f64>().await;
    i.stack_push(a.into_inner() as i64).await;
}

/// Install numeric functions for i64 and f64
pub fn install(i: &mut Interpreter) {
    i.define("i64?", util::type_predicate::<i64>);
    i.define("f64?", util::type_predicate::<f64>);

    i.define("i64-equal", util::equality::<i64>);
    i.define("f64-equal", util::equality::<f64>);

    i.define("i64-add", add::<i64>);
    i.define("f64-add", add::<f64>);
    i.define("i64-sub", sub::<i64>);
    i.define("f64-sub", sub::<f64>);
    i.define("i64-mul", mul::<i64>);
    i.define("f64-mul", mul::<f64>);
    i.define("i64-div", div::<i64>);
    i.define("f64-div", div::<f64>);

    i.define("i64-negate", negate::<i64>);
    i.define("f64-negate", negate::<f64>);

    // quotient
    // remainder
    // quotient/  remainder
    // modulo

    i.define("i64-abs", abs_i64);
    i.define("f64-abs", abs_f64);

    i.define("i64-lt", lt::<i64>);
    i.define("f64-lt", lt::<f64>);
    i.define("i64-le", le::<i64>);
    i.define("f64-le", le::<f64>);
    i.define("i64-gt", gt::<i64>);
    i.define("f64-gt", gt::<f64>);
    i.define("i64-ge", ge::<i64>);
    i.define("f64-ge", ge::<f64>);

    i.define("i64->f64", i64_to_f64);
    i.define("f64->i64", f64_to_i64);
}

