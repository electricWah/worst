
use crate::list::*;
use crate::interpreter::{Interpreter, Handle};

pub async fn list_empty(mut i: Handle) {
    let l = i.stack_pop::<List>().await;
    let len = l.len();
    i.stack_push(l).await;
    i.stack_push(len == 0).await;
}

pub async fn list_length(mut i: Handle) {
    let l = i.stack_pop::<List>().await;
    let len = l.len();
    i.stack_push(l).await;
    i.stack_push(len as i32).await;
}

pub async fn list_push(mut i: Handle) {
    let v = i.stack_pop_val().await;
    let mut l = i.stack_pop::<List>().await;
    l.push(v);
    i.stack_push(l).await;
}

pub async fn list_pop(mut i: Handle) {
    let mut l = i.stack_pop::<List>().await;
    let v = l.pop().unwrap_or(false.into());
    i.stack_push(l).await;
    i.stack_push(v).await;
}

pub async fn list_reverse(mut i: Handle) {
    let mut l = i.stack_pop::<List>().await;
    l.reverse();
    i.stack_push(l).await;
}

pub fn install(mut i: Interpreter) -> Interpreter {
    i.define("list-empty?", list_empty);
    i.define("list-length", list_pop);
    i.define("list-reverse", list_reverse);
    i.define("list-push", list_push);
    i.define("list-pop", list_pop);
    i
}

