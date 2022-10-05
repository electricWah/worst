
//! Places are mutable things that can each store a value

use crate::interpreter::{Interpreter, Handle};
use crate::base::*;

/// Install `make-place`, `place-get` and `place-set` functions.
pub fn install(i: &mut Interpreter) {
    i.define("make-place", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        i.stack_push(Place::wrap(v)).await;
    });
    i.define("place-get", |mut i: Handle| async move {
        let v = i.stack_pop::<Place>().await.as_ref().get();
        i.stack_push(v).await;
    });
    i.define("place-set", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        let mut p = i.stack_pop::<Place>().await;
        p.as_mut().set(v);
        i.stack_push(p).await;
    });
}

