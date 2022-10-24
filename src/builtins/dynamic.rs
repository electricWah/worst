
//! Dynamic values.

use crate::base::*;
use crate::interpreter::{Interpreter, Handle};

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.define("dynamic-resolve", |mut i: Handle| async move {
        let name = i.stack_pop::<Symbol>().await.into_inner();
        let res = i.resolve_dynamic(name).await;
        match res {
            Some(def) => i.stack_push(def).await,
            None => i.stack_push(false).await,
        }
    });
}

