
use crate::list::*;
use crate::interpreter::{Interpreter, Handle};

pub async fn is_list(mut i: Handle) {
    let l = i.stack_top_val().await;
    i.stack_push(l.is::<List>()).await;
}

pub async fn list_empty(mut i: Handle) {
    let l = i.stack_top::<List>().await;
    i.stack_push(l.as_ref().is_empty()).await;
}

pub async fn list_length(mut i: Handle) {
    let l = i.stack_top::<List>().await;
    i.stack_push(l.as_ref().len() as i32).await;
}

pub async fn list_push(mut i: Handle) {
    let v = i.stack_pop_val().await;
    let mut l = i.stack_pop::<List>().await;
    l.as_mut().push(v);
    i.stack_push(l).await;
}

pub async fn list_pop(mut i: Handle) {
    let mut l = i.stack_pop::<List>().await;
    let v = l.as_mut().pop().unwrap_or_else(|| false.into());
    i.stack_push(l).await;
    i.stack_push(v).await;
}

pub async fn list_reverse(mut i: Handle) {
    let mut l = i.stack_pop::<List>().await;
    l.as_mut().reverse();
    i.stack_push(l).await;
}

pub async fn list_append(mut i: Handle) {
    let mut b = i.stack_pop::<List>().await;
    let a = i.stack_pop::<List>().await;
    b.as_mut().prepend(a.into_inner());
    i.stack_push(b).await;
}

pub fn install(i: &mut Interpreter) {
    i.define("list?", is_list);
    i.define("list-empty?", list_empty);
    i.define("list-length", list_pop);
    i.define("list-reverse", list_reverse);
    i.define("list-push", list_push);
    i.define("list-pop", list_pop);
    i.define("list-append", list_append);
}

