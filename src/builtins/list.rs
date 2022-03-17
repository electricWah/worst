
use match_downcast::*;
use crate::base::*;
use crate::list::*;
use crate::interpreter::{Builtin, Builder, Handle};

async fn expect_list(i: &mut Handle) -> Option<List<Val>> {
    if let Some(l) = i.stack_pop::<List<Val>>().await {
        Some(l)
    } else if i.stack_empty().await {
        i.stack_push("stack-empty".to_symbol()).await;
        None
    } else {
        i.stack_push("wrong-type".to_symbol()).await;
        None
    }
}

pub async fn list_empty(mut i: Handle) {
    if let Some(l) = expect_list(&mut i).await {
        let len = l.len();
        i.stack_push(l).await;
        i.stack_push(len == 0).await;
    } else {
        return i.pause().await;
    }
}

pub async fn list_pop(mut i: Handle) {
    if let Some(mut l) = expect_list(&mut i).await {
        let v = l.pop().unwrap_or(false.into());
        i.stack_push(l).await;
        i.stack_push(v).await;
    } else {
        return i.pause().await;
    }
}


pub fn install(mut i: Builder) -> Builder {
    i.define("list-empty?", list_empty);
    i.define("list-pop", list_pop);
    i
}

