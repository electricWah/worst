
//! Stuff you might like to use when defining builtins.

use std::fmt::Debug;
use crate::base::*;
use crate::interpreter::Handle;

/// Type predicate wrapper, e.g.
/// ```ignore
/// i.define("string?", type_predicate::<String>);
/// ```
pub async fn type_predicate<T: Value>(mut i: Handle) {
    let v = i.stack_top_val().await;
    i.stack_push(v.is::<T>()).await;
}

/// Equality generator, e.g.
/// ```ignore
/// i.define("string-equal", equality::<String>);
/// ```
pub async fn equality<T: Value + PartialEq>(mut i: Handle) {
    let b = i.stack_pop::<T>().await;
    let a = i.stack_pop::<T>().await;
    i.stack_push(a.as_ref() == b.as_ref()).await;
}

/// Debug to-string generator, e.g.
/// ```ignore
/// i.define("i64->string", value_tostring_debug::<i64>);
/// ```
/// ```ignore
/// ; i64 i64->string -> string
/// 11 i64->string ; -> "11"
/// ```
pub async fn value_tostring_debug<T: Value + Debug>(mut i: Handle) {
    let v = i.stack_pop::<T>().await;
    i.stack_push(format!("{:?}", v.as_ref())).await;
}

