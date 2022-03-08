
use crate::base::*;
use crate::interpreter::{Builder, Handle};

pub async fn doc(mut i: Handle) {
    if let Some(q) = i.quote().await {
        dbg!("doc", q);
    } else {
        i.stack_push("quote-nothing".to_symbol()).await;
        return i.pause().await;
    }
}

pub fn install(i: Builder) -> Builder {
    i.define("doc", doc)
}


