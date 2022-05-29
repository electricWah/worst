
use crate::interpreter::{Interpreter, Handle};

pub async fn doc(mut i: Handle) {
    let _q = i.quote_val().await;
    // dbg!("doc added");
}

pub fn install(mut i: Interpreter) -> Interpreter {
    i.define("doc", doc);
    i
}


