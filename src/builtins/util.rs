
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

