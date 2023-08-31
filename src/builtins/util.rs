
//! Stuff you might like to use when defining builtins.

use std::fmt::Debug;
use std::io::{ Read, Write };
use crate::base::*;
use crate::interpreter::*;

/// Make a builtin that pushes default() to the stack
pub fn make_default<T: Value + Default>(i: &mut Interpreter) -> BuiltinRet {
    i.stack_push(T::default());
    Ok(())
}

/// Add a builtin to just put the TypeId of the given type on the stack.
/// The TypeId value will have a String metadata containing its name.
pub fn add_const_type_builtin<T: Value>(i: &mut Interpreter, name: impl Into<String>) {
    let name = name.into();
    let mut t = Val::from(TypeId::of::<T>());
    t.meta_mut().insert_val(i.uniques_mut().get_type::<String>(), name.clone().into());
    i.add_definition(name, t);
}

/// Equality generator, e.g.
/// ```ignore
/// i.define("string-equal", equality::<String>);
/// ```
pub fn equality<T: Value + PartialEq>(i: &mut Interpreter) -> BuiltinRet {
    let b = i.stack_pop::<T>()?;
    let a = i.stack_pop::<T>()?;
    i.stack_push(a.as_ref() == b.as_ref());
    Ok(())
}

/// Comparison generator, e.g.
/// ```ignore
/// i.define("string-compare", comparison::<String>);
/// ```
/// a b comparison ->
/// -1 when a < b, 0 when a == b, 1 when a > b,
/// false when unavailable
pub fn comparison<T: Value + PartialOrd>(i: &mut Interpreter) -> BuiltinRet {
    let b = i.stack_pop::<T>()?;
    let a = i.stack_pop::<T>()?;
    i.stack_push_option(a.as_ref().partial_cmp(b.as_ref()).map(|o| o as i64));
    Ok(())
}

/// Debug to-string generator, e.g.
/// ```ignore
/// i.define("i64->string", value_tostring_debug::<i64>);
/// ```
/// ```ignore
/// ; i64 i64->string -> string
/// 11 i64->string ; -> "11"
/// ```
pub fn value_tostring_debug<T: Value + Debug>(i: &mut Interpreter) -> BuiltinRet {
    let v = i.stack_pop::<T>()?;
    i.stack_push(format!("{:?}", v.as_ref()));
    Ok(())
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
pub fn or_io_error<T>(i: &mut Interpreter, e: std::io::Result<T>) -> Option<T> {
    match e {
        Ok(v) => Some(v),
        Err(e) => {
            i.stack_push_error(format!("{}", e));
            None
        }
    }
}

/// Slurp entire port (i.e. until eof) into a string.
/// Pops a `T` and pushes the result,
/// either the string itself on success,
/// or an `error?` value on failure
/// (currently the string representation of the error).
pub fn port_to_string<T: Value + Read + Clone>(i: &mut Interpreter) -> BuiltinRet {
    let mut read = i.stack_pop::<T>()?;
    let mut s = String::new();
    if let Some(_count) = or_io_error(i, read.as_mut().read_to_string(&mut s)) {
        i.stack_push(s);
    }
    Ok(())
}

/// Read a specified range from a port into a bytevector.
/// Creates a builtin with the following signature:
/// `port bytevector start end port-read-bytevector-range -> port bytevector read-count-or-error`
/// See [bytes_range_mut] for da rulez.
pub fn port_read_range<T: Value + Read + Clone>(i: &mut Interpreter) -> BuiltinRet {
    let end = i.stack_pop::<i64>()?.into_inner();
    let start = i.stack_pop::<i64>()?.into_inner();
    let mut bytevector = i.stack_pop::<Vec<u8>>()?;
    let mut port = i.stack_pop::<T>()?;
    let bv = bytevector.as_mut();
    let range = bytes_range_mut(bv, start, end);
    let res = 
        port.as_mut().read(range)
        .map(|count| Val::from(count as i64))
        .map_err(|e| format!("{}", e));
    i.stack_push(port);
    i.stack_push(bytevector);
    i.stack_push_result(res);
    Ok(())
}

/// Write a specified range from a bytevector into a port.
/// Creates a builtin with the following signature:
/// `port bytevector start end port-write-bytevector-range -> port bytevector write-count-or-error`
/// See [bytes_range_mut] for da rulez.
pub fn port_write_range<T: Value + Write + Clone>(i: &mut Interpreter) -> BuiltinRet {
    let end = i.stack_pop::<i64>()?.into_inner();
    let start = i.stack_pop::<i64>()?.into_inner();
    let bytevector = i.stack_pop::<Vec<u8>>()?;
    let mut port = i.stack_pop::<T>()?;
    let range = bytes_range(bytevector.as_ref(), start, end);
    let res = port.as_mut().write(range)
        .map(|count| Val::from(count as i64))
        .map_err(|e| format!("{}", e));
    i.stack_push(port);
    i.stack_push(bytevector);
    i.stack_push_result(res);
    Ok(())
}

/// Write a string to a port.
/// Creates a builtin with the following signature:
/// `port string port-write-string -> port`
// TODO handle write failure
pub fn port_write_string<T: Value + Write + Clone>(i: &mut Interpreter) -> BuiltinRet {
    let str = i.stack_pop::<String>()?;
    let mut port = i.stack_pop::<T>()?;
    let _todo_handle = port.as_mut().write(str.as_ref().as_ref()).unwrap();
    i.stack_push(port);
    Ok(())
}

/// Flush an output port.
/// `port port-flush -> port true-or-error`
pub fn port_flush<T: Value + Write + Clone>(i: &mut Interpreter) -> BuiltinRet {
    let mut p = i.stack_pop::<T>()?;
    let r = p.as_mut().flush();
    i.stack_push(p);
    match r {
        Ok(()) => i.stack_push(true),
        Err(e) => i.stack_push_error(format!("{}", e)),
    }
    Ok(())
}

