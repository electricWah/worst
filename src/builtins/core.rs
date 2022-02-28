
use crate::interpreter::{Builder, Handle};

pub fn install(i: Builder) -> Builder {
    i.define("quote", |mut i: Handle| async move {
        let q = i.quote().await.unwrap_or(false.into());
        i.stack_push(q).await;
    }).define("stack-dump", |mut i: Handle| async move {
        dbg!(i.stack_get().await);
    })
}

