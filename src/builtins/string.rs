
use crate::base::*;
use crate::interpreter::{Builder, Handle};

pub fn install(mut i: Builder) -> Builder {
    i.define("string-append", |mut i: Handle| async move {
        let b = i.stack_pop::<String>().await;
        let mut a = i.stack_pop::<String>().await;
        a.push_str(&b);
        i.stack_push(a).await;
    });
    i.define("whitespace?", |mut i:Handle| async move {
        let s = i.stack_pop::<String>().await;
        let ws = s.chars().all(char::is_whitespace);
        i.stack_push(s).await;
        i.stack_push(ws).await;
    });
    i.define("string->symbol", |mut i: Handle| async move {
        let s = i.stack_pop::<String>().await;
        i.stack_push(s.to_symbol()).await;
    });
    i
}

