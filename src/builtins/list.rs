
//! [List](crate::list::List) manipulation basics

use crate::list::*;
use crate::interpreter::{Interpreter, Handle};

/// any `list?` +-> bool : whether the value on top of the stack is a list.
pub async fn is_list(mut i: Handle) {
    let l = i.stack_top_val().await;
    i.stack_push(l.is::<List>()).await;
}

/// list `list-empty?` +-> bool : whether the list on top of the stack is empty.
pub async fn list_empty(mut i: Handle) {
    let l = i.stack_top::<List>().await;
    i.stack_push(l.as_ref().is_empty()).await;
}

/// list `list-empty?` +-> i64 : the length of the list.
pub async fn list_length(mut i: Handle) {
    let l = i.stack_top::<List>().await;
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

/// list list `list-reverse` -> list : append two lists.
pub async fn list_append(mut i: Handle) {
    let mut b = i.stack_pop::<List>().await;
    let a = i.stack_pop::<List>().await;
    b.as_mut().prepend(a.into_inner());
    i.stack_push(b).await;
}

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.define("list?", is_list);
    i.define("list-empty?", list_empty);
    i.define("list-length", list_pop);
    i.define("list-reverse", list_reverse);
    i.define("list-push", list_push);
    i.define("list-pop", list_pop);
    i.define("list-append", list_append);
}

