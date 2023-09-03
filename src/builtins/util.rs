
//! Stuff you might like to use when defining builtins.

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

/// Turn the value into a String via [format!].
pub fn display<T: std::fmt::Display>(v: T) -> String { format!("{}", v) }

