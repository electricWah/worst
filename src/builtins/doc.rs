
use crate::base::*;
use crate::interpreter::{Builder, Handle};

pub async fn doc(mut i: Handle) {
    if let Some(_q) = i.quote().await {
        dbg!("doc added");
    } else {
        i.stack_push("quote-nothing".to_symbol()).await;
        return i.pause().await;
    }
}

pub fn install(mut i: Builder) -> Builder {
    i.define("doc", doc);
    i
}


