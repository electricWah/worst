
use crate::reader::*;
use crate::interpreter::{Builder, Handle};

pub fn install(mut i: Builder) -> Builder {
    i.define("new-reader", |mut i: Handle| async move {
        i.stack_push(Reader::new()).await;
    });
    i.define("reader-set-eof", |mut i: Handle| async move {
        let mut r = i.stack_pop::<Reader>().await;
        r.set_eof();
        i.stack_push(r).await;
    });
    i.define("reader-write-string", |mut i: Handle| async move {
        let s = i.stack_pop::<String>().await;
        let mut r = i.stack_pop::<Reader>().await;
        r.write(&mut s.chars());
        i.stack_push(r).await;
    });
    // -> val #t | err #f | #f #f (eof)
    i.define("reader-next", |mut i: Handle| async move {
        let mut r = i.stack_pop::<Reader>().await;
        let res = r.next();
        i.stack_push(r).await;
        match res {
            Ok(Some(v)) => {
                i.stack_push(v).await;
                i.stack_push(true).await;
            },
            Ok(None) => {
                i.stack_push(false).await;
                i.stack_push(false).await;
            },
            Err(e) => {
                i.stack_push(e).await;
                i.stack_push(false).await;
            },
        }
    });
    i
}

