
use crate::base::*;
use crate::list::*;
use crate::interpreter::{Builder, Handle};

pub async fn list_empty(mut i: Handle) {
    let l = i.stack_pop::<List<Val>>().await;
    let len = l.len();
    i.stack_push(l).await;
    i.stack_push(len == 0).await;
}

pub async fn list_pop(mut i: Handle) {
    let mut l = i.stack_pop::<List<Val>>().await;
    let v = l.pop().unwrap_or(false.into());
    i.stack_push(l).await;
    i.stack_push(v).await;
}

pub async fn list_reverse(mut i: Handle) {
    let mut l = i.stack_pop::<List<Val>>().await;
    l.reverse();
    i.stack_push(l).await;
}

pub fn install(mut i: Builder) -> Builder {
    i.define("list-empty?", list_empty);
    i.define("list-reverse", list_reverse);
    i.define("list-pop", list_pop);
    i
}

