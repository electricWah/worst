
//! A bytevector is a bunch of bytes (a [Vec<u8>]).
//! Use it to concatenate a lot of strings,
//! read or write data and such,
//! add (line-)buffering to an input port,
//! and whatever else it is you want to do with a bunch of bytes.

use crate::base::*;
use super::util;
use crate::interpreter::{Interpreter, Handle};

impl Value for Vec<u8> {}

/// Install some bytevector definitions.
pub fn install(i: &mut Interpreter) {
    i.define("bytevector?", util::type_predicate::<Vec<u8>>);
    i.define("bytevector-equal", util::equality::<Vec<u8>>);
    i.define("bytevector-length", |mut i: Handle| async move {
        let v = i.stack_top::<Vec<u8>>().await;
        i.stack_push(v.as_ref().len() as i64).await;
    });
    // ??? bytevector/string/i8/u8/i32/f32/...
    // i.define("bytevector-get", |mut i: Handle| async move {
    // });
    // i.define("bytevector-set", |mut i: Handle| async move {
    // });

    // bv start len bytevector-range -> bv
    // combination truncate + extend + substring
    // if start > 0, remove < start bytes
    // if len goes beyond end, pad with zeroes
    i.define("bytevector-range", |mut i: Handle| async move {
        let len = i.stack_pop::<i64>().await.into_inner();
        let start = i.stack_pop::<i64>().await.into_inner();
        let mut v = i.stack_pop::<Vec<u8>>().await;
        let vlen = v.as_ref().len() as i64;
        let start = start.max(0).min(vlen) as usize;
        let len = len.max(0) as usize;
        if len == 0 {
            (*v.as_mut()) = vec![];
        } else if start > 0 {
            let vmut = v.as_mut();
            let mut newv = vmut.split_off(start);
            std::mem::swap(vmut, &mut newv);
        }
        v.as_mut().resize(len, 0);
        i.stack_push(v).await;
    });

    i.define("bytevector-split", |mut i: Handle| async move {
        let idx = i.stack_pop::<i64>().await.into_inner();
        let mut a = i.stack_pop::<Vec<u8>>().await;
        let idx = idx.max(0).min(a.as_ref().len() as i64);
        let b = a.as_mut().split_off(idx as usize);
        i.stack_push(b).await;
        i.stack_push(a).await;
    });

    // i.define("bytevector-", |mut i: Handle| async move {
    // });
    i.define("string->bytevector", |mut i: Handle| async move {
        let s = i.stack_pop::<String>().await.into_inner();
        i.stack_push(Vec::<u8>::from(s.as_str())).await;
    });
}

