
use crate::interpreter::{Builder, Handle};

pub async fn doc(mut i: Handle) {
    let _q = i.quote_val().await;
    // dbg!("doc added");
}

pub fn install(mut i: Builder) -> Builder {
    i.define("doc", doc);
    i
}


