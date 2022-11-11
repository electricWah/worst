
//! Stuff you might like to use when defining builtins.

use std::fmt::Debug;
use std::io::{ Read, Write };
use crate::base::*;
use crate::interpreter::Handle;

/// Type predicate wrapper, e.g.
/// ```ignore
/// i.define("string?", type_predicate::<String>);
/// ```
pub async fn type_predicate<T: Value>(mut i: Handle) {
    let v = i.stack_top_val().await;
    i.stack_push(v.is::<T>()).await;
}

/// Equality generator, e.g.
/// ```ignore
/// i.define("string-equal", equality::<String>);
/// ```
pub async fn equality<T: Value + PartialEq>(mut i: Handle) {
    let b = i.stack_pop::<T>().await;
    let a = i.stack_pop::<T>().await;
    i.stack_push(a.as_ref() == b.as_ref()).await;
}

/// Debug to-string generator, e.g.
/// ```ignore
/// i.define("i64->string", value_tostring_debug::<i64>);
/// ```
/// ```ignore
/// ; i64 i64->string -> string
/// 11 i64->string ; -> "11"
/// ```
pub async fn value_tostring_debug<T: Value + Debug>(mut i: Handle) {
    let v = i.stack_pop::<T>().await;
    i.stack_push(format!("{:?}", v.as_ref())).await;
}

/// Get an index within a 0..len range (optionally extend beyond len)
pub fn index_range(len: usize, idx: i64, extend: bool) -> usize {
    let r = (if idx < 0 { len as i64 + idx } else { idx }).max(0) as usize;
    if extend { r } else { r.min(len) }
}

/// Get a start..end range within slice
// len < 0 could swap start/end positions? or go from the end of the vec?
pub fn get_range(slice: &[u8], start: i64, end: i64, extend: bool) -> (usize, usize) {
    let vlen = slice.len();
    let start = index_range(vlen, start, false);
    let end = index_range(vlen, end, extend);
    (start, end)
}

/// Get a reference to a range of bytes in a vector.
/// If start or end < 0, they are counted from the end.
/// Then they are clipped within bounds.
/// The returned slice may be shorter than requested if end > bytes.len().
pub fn bytes_range(bytes: &[u8], start: i64, end: i64) -> &[u8] {
    let (start, end) = get_range(bytes, start, end, false);
    &bytes[start .. end]
}

/// Get a mutable reference to a range of bytes in a vector. See [bytes_range]
pub fn bytes_range_mut(bytes: &mut [u8], start: i64, end: i64) -> &mut [u8] {
    let (start, end) = get_range(bytes, start, end, false);
    &mut bytes[start .. end]
}

/// Given an [io::Result], either return its `Ok` arm or put the error on the stack.
/// You should push one value to the stack in the `Some` return case.
pub async fn or_io_error<T>(i: &mut Handle, e: std::io::Result<T>) -> Option<T> {
    match e {
        Ok(v) => Some(v),
        Err(e) => {
            i.stack_push(IsError::add(format!("{}", e))).await;
            None
        }
    }
}

/// Slurp entire port (i.e. until eof) into a string.
/// Pops a `T` and pushes the result,
/// either the string itself on success,
/// or an `error?` value on failure
/// (currently the string representation of the error).
pub async fn port_to_string<T: Value + Read + Clone>(mut i: Handle) {
    let mut read = i.stack_pop::<T>().await;
    let mut s = String::new();
    if let Some(_count) = or_io_error(&mut i, read.as_mut().read_to_string(&mut s)).await {
        i.stack_push(s).await;
    }
}

/// Read a specified range from a port into a bytevector.
/// Creates a builtin with the following signature:
/// `port bytevector start end port-read-bytevector-range -> port bytevector read-count-or-error`
/// See [bytes_range_mut] for da rulez.
pub async fn port_read_range<T: Value + Read + Clone>(mut i: Handle) {
    let end = i.stack_pop::<i64>().await.into_inner();
    let start = i.stack_pop::<i64>().await.into_inner();
    let mut bytevector = i.stack_pop::<Vec<u8>>().await;
    let mut port = i.stack_pop::<T>().await;
    let bv = bytevector.as_mut();
    let range = bytes_range_mut(bv, start, end);
    let res = 
        match port.as_mut().read(range) {
            Ok(count) => Val::from(count as i64),
            Err(e) => IsError::add(format!("{}", e)),
        };
    i.stack_push(port).await;
    i.stack_push(bytevector).await;
    i.stack_push(res).await;
}

/// Write a specified range from a bytevector into a port.
/// Creates a builtin with the following signature:
/// `port bytevector start end port-write-bytevector-range -> port bytevector write-count-or-error`
/// See [bytes_range_mut] for da rulez.
pub async fn port_write_range<T: Value + Write + Clone>(mut i: Handle) {
    let end = i.stack_pop::<i64>().await.into_inner();
    let start = i.stack_pop::<i64>().await.into_inner();
    let bytevector = i.stack_pop::<Vec<u8>>().await;
    let mut port = i.stack_pop::<T>().await;
    let range = bytes_range(bytevector.as_ref(), start, end);
    let res =
        match port.as_mut().write(range) {
            Ok(count) => Val::from(count as i64),
            Err(e) => IsError::add(format!("{}", e)),
        };
    i.stack_push(port).await;
    i.stack_push(bytevector).await;
    i.stack_push(res).await;
}

/// Write a string to a port.
/// Creates a builtin with the following signature:
/// `port string port-write-string -> port`
// TODO handle write failure
pub async fn port_write_string<T: Value + Write + Clone>(mut i: Handle) {
    let str = i.stack_pop::<String>().await;
    let mut port = i.stack_pop::<T>().await;
    let _todo_handle = port.as_mut().write(str.as_ref().as_ref()).unwrap();
    i.stack_push(port).await;
}

/// Flush an output port.
/// `port port-flush -> port true-or-error`
pub async fn port_flush<T: Value + Write + Clone>(mut i: Handle) {
    let mut p = i.stack_pop::<T>().await;
    let r = p.as_mut().flush();
    i.stack_push(p).await;
    match r {
        Ok(()) => i.stack_push(true).await,
        Err(e) => i.stack_push(IsError::add(format!("{}", e))).await,
    }
}

