
//! Stuff you might like to use when defining builtins.

use crate::base::*;
use crate::interpreter::Handle;

/// Type predicate wrapper, e.g.
/// ```ignore
/// i.define("string?", type_predicate::<String>);
/// ```
pub async fn type_predicate<T: ImplValue + 'static>(mut i: Handle) {
    let v = i.stack_top_val().await;
    i.stack_push(v.is::<T>()).await;
}

/// Equality generator, e.g.
/// ```ignore
/// i.define("string-equal", equality::<String>);
/// ```
pub async fn equality<T: ImplValue + 'static + PartialEq>(mut i: Handle) {
    let b = i.stack_pop::<T>().await;
    let a = i.stack_pop::<T>().await;
    i.stack_push(a.as_ref() == b.as_ref()).await;
}

