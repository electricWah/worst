
use crate::interpreter::{Interpreter, Handle};

pub fn install(i: &mut Interpreter) {
    i.define("get-environment-variable", |mut i: Handle| async move {
        match std::env::var(i.stack_top::<String>().await.as_ref()) {
            Ok(v) => i.stack_push(v).await,
            Err(_) => i.stack_push(false).await,
        }
    });
}
