
use crate::interpreter::{Builder, Handle};
use crate::base::*;

pub fn install(mut i: Builder) -> Builder {
    i.define("make-place", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        i.stack_push(Place::wrap(v)).await;
    });
    i.define("place-get", |mut i: Handle| async move {
        let p = i.stack_pop::<Place>().await;
        let v = p.get();
        i.stack_push(p).await;
        i.stack_push(v).await;
    });
    i.define("place-set", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        let mut p = i.stack_pop::<Place>().await;
        p.set(v);
        i.stack_push(p).await;
    });

    i
}

