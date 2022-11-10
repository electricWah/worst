
//! A bytevector is a bunch of bytes (a [Vec<u8>]).
//! Use it to concatenate a lot of strings,
//! read or write data and such,
//! add (line-)buffering to an input port,
//! and whatever else it is you want to do with a bunch of bytes.

use crate::base::*;
use super::util;
use crate::interpreter::{Interpreter, Handle};

impl Value for Vec<u8> {}

fn index_range(len: usize, idx: i64, extend: bool) -> usize {
    let r = (if idx < 0 { len as i64 + idx } else { idx }).max(0) as usize;
    if extend { r } else { r.min(len) }
}

// len < 0 could swap start/end positions? or go from the end of the vec?
fn get_range(slice: &[u8], start: i64, end: i64, extend: bool) -> (usize, usize) {
    let vlen = slice.len();
    let start = index_range(vlen, start, false);
    let end = index_range(vlen, end, extend);
    (start, end)
}

/// Get a reference to a range of bytes in a vector.
/// If start or end < 0, they are counted from the end.
/// Then they are clipped within bounds.
/// The returned slice may be shorter than requested if end > bytes.len().
pub fn bytes_range(bytes: &Vec<u8>, start: i64, end: i64) -> &[u8] {
    let (start, end) = get_range(&bytes, start, end, false);
    &bytes[start .. end]
}

/// Get a mutable reference to a range of bytes in a vector. See [bytes_range]
pub fn bytes_range_mut(bytes: &mut Vec<u8>, start: i64, end: i64) -> &mut [u8] {
    let (start, end) = get_range(&bytes, start, end, false);
    &mut bytes[start .. end]
}

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
    // if start < 0, take from end
    // if start > 0, remove < start bytes
    // if len goes beyond end, pad with zeroes
    i.define("bytevector-range", |mut i: Handle| async move {
        let end = i.stack_pop::<i64>().await.into_inner();
        let start = i.stack_pop::<i64>().await.into_inner();
        let mut v = i.stack_pop::<Vec<u8>>().await;
        let (start, end) = get_range(v.as_ref(), start, end, true);
        if start == end {
            (*v.as_mut()) = vec![];
        } else {
            let vmut = v.as_mut();
            let mut newv = vmut.split_off(start);
            std::mem::swap(vmut, &mut newv);
        }
        v.as_mut().resize(end - start, 0);
        i.stack_push(v).await;
    });

    i.define("bytevector-split", |mut i: Handle| async move {
        let idx = i.stack_pop::<i64>().await.into_inner();
        let mut a = i.stack_pop::<Vec<u8>>().await;
        let idx = index_range(a.as_ref().len(), idx, false);
        let b = a.as_mut().split_off(idx);
        i.stack_push(b).await;
        i.stack_push(a).await;
    });

    i.define("string-utf8->bytevector", |mut i: Handle| async move {
        let s = i.stack_pop::<String>().await.into_inner();
        i.stack_push(Vec::<u8>::from(s.as_str())).await;
    });
    i.define("bytevector->string-utf8", |mut i: Handle| async move {
        let bv = i.stack_pop::<Vec<u8>>().await.into_inner();
        match String::from_utf8(bv) {
            Ok(s) => i.stack_push(s).await,
            Err(e) => i.stack_push(IsError::add(format!("{}", e))).await,
        }
    });
}

