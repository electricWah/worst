
//! [List](crate::list::List) manipulation basics

use crate::base::*;
use crate::builtins::util;
use crate::interpreter::{Interpreter, Handle};

/// list `list-length` -> i64 : the length of the list.
pub async fn list_length(mut i: Handle) {
    let l = i.stack_pop::<List>().await;
    i.stack_push(l.as_ref().len() as i64).await;
}

/// list val `list-push` -> list : put the value at the front of the list.
pub async fn list_push(mut i: Handle) {
    let v = i.stack_pop_val().await;
    let mut l = i.stack_pop::<List>().await;
    l.as_mut().push(v);
    i.stack_push(l).await;
}

/// list `list-pop` +-> val : take the front value off the list (or false).
pub async fn list_pop(mut i: Handle) {
    let mut l = i.stack_pop::<List>().await;
    let v = l.as_mut().pop().unwrap_or_else(|| false.into());
    i.stack_push(l).await;
    i.stack_push(v).await;
}

/// list `list-reverse` -> list : reverse the list.
pub async fn list_reverse(mut i: Handle) {
    let mut l = i.stack_pop::<List>().await;
    l.as_mut().reverse();
    i.stack_push(l).await;
}

/// list list `list-append` -> list : append two lists.
pub async fn list_append(mut i: Handle) {
    let mut b = i.stack_pop::<List>().await;
    let a = i.stack_pop::<List>().await;
    b.as_mut().prepend(a.into_inner());
    i.stack_push(b).await;
}

/// list n `list-get` -> value : get the value at index n of list.
/// 0-indexed, negative numbers are from the other end of the list,
/// and out of range gives false with error? as true.
pub async fn list_get(mut i: Handle) {
    let n = i.stack_pop::<i64>().await;
    let l = i.stack_pop::<List>().await;
    let n = n.into_inner();
    let l = l.as_ref();
    let n = if n < 0 { l.len() as i64 + n } else { n };
    i.stack_push_result(l.get(n as usize).cloned().ok_or(false)).await;
}

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.define("list?", util::type_predicate::<List>);
    i.define("list-length", list_length);
    i.define("list-reverse", list_reverse);
    i.define("list-push", list_push);
    i.define("list-pop", list_pop);
    i.define("list-append", list_append);
    i.define("list-get", list_get);
}

