
use crate::base::*;
use crate::interpreter::{Builder, Handle};

pub fn install(mut i: Builder) -> Builder {
    i.define("string-append", |mut i: Handle| async move {
        let b = i.stack_pop::<String>().await;
        let mut a = i.stack_pop::<String>().await;
        a.push_str(&b);
        i.stack_push(a).await;
    });
    i
}

