
//! Numeric operations that work on i64 and f64 numbers

use super::util;
use crate::base::*;
use crate::interpreter::{Interpreter, Handle};

// TODO pop two at once with stack_pop::<(T, T)>
/// Add two numbers together.
pub async fn add<T: std::ops::Add<T, Output=T> + ImplValue + Clone + 'static>(mut i: Handle) {
    let b = i.stack_pop::<T>().await;
    let a = i.stack_pop::<T>().await;
    i.stack_push(a.into_inner() + b.into_inner()).await;
}

/// Install numeric functions for i64 and f64
pub fn install(i: &mut Interpreter) {
    i.define("i64?", util::type_predicate::<i64>);
    i.define("f64?", util::type_predicate::<f64>);
    i.define("i64-add", add::<i64>);
    i.define("f64-add", add::<f64>);
}

