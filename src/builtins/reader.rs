
use crate::base::*;
use crate::list::*;
use crate::reader::*;
use crate::interpreter::{Interpreter, Handle};

pub fn install(i: &mut Interpreter) {
    i.define("reader-empty", |mut i: Handle| async move {
        i.stack_push(Reader::new()).await;
    });
    i.define("reader-set-eof", |mut i: Handle| async move {
        let mut r = i.stack_pop::<Reader>().await;
        r.as_mut().set_eof();
        i.stack_push(r).await;
    });
    i.define("reader-write-string", |mut i: Handle| async move {
        let mut s = i.stack_pop::<String>().await;
        let mut r = i.stack_pop::<Reader>().await;
        r.as_mut().write(&mut s.as_mut().chars());
        i.stack_push(r).await;
    });
    // -> val #t | err #f | #f #f (eof)
    i.define("reader-next", |mut i: Handle| async move {
        let mut r = i.stack_pop::<Reader>().await;
        let res = r.as_mut().read_next();
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
    // Read an entire port to the end into a list (or other, error, value)
    i.define("read-port->list", |mut i: Handle| async move {
        let pv = i.stack_pop_val().await;
        if let Some(mut read) = ReadValue::try_read(pv) {
            let mut s = String::new();
            match read.read_to_string(&mut s) {
                Ok(_count) => match read_all(&mut s.chars()) {
                    Ok(v) => i.stack_push(List::from(v)).await,
                    Err(e) => i.stack_push(format!("{:?}", e)).await,
                },
                Err(e) => {
                    i.stack_push(format!("{}", e)).await;
                },
            }
        } else {
            todo!("not a port etc");
        }
    });
}

