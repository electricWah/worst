
//! Ports: anything implementing [std::io::Read] or [std::io::Write].

use crate::interpreter::*;
use super::util;
use std::io;

/// `port-input?` - Can the value be interpreted as an input port ([io::Read])?
/// Builtins that try to read from a port will push error #f otherwise.
pub fn port_input(i: &mut Interpreter) -> BuiltinRet {
    // as_trait_mut otherwise the clone might fail
    let mut v = i.stack_pop_val()?;
    let can = v.as_trait_mut::<dyn io::Read>().is_some();
    i.stack_push(v);
    i.stack_push(can);
    Ok(())
}

/// `port-output?` - Can the value be interpreted as an output port ([io::Write])?
pub fn port_output(i: &mut Interpreter) -> BuiltinRet {
    let mut v = i.stack_pop_val()?;
    let can = v.as_trait_mut::<dyn io::Write>().is_some();
    i.stack_push(v);
    i.stack_push(can);
    Ok(())
}

/// `port-read->string` - Read entire port (i.e. until eof) into a string.
/// Pushes either the string itself on success,
/// or an `error?` value on failure
/// (#f if not a readable port, or an IO error message as a string).
pub fn port_read_to_string(i: &mut Interpreter) -> BuiltinRet {
    let mut port = i.stack_pop_val()?;
    let Some(p) = port.as_trait_mut::<dyn io::Read>() else {
        i.stack_push(port);
        i.stack_push_error(false);
        return Ok(());
    };
    let mut s = String::new();
    let res = p.read_to_string(&mut s).map(|_| s).map_err(util::display);
    i.stack_push(port);
    i.stack_push_result(res);
    Ok(())
}

/// `port string port-write-string -> port count-bytes-written|error` -
/// Write a string to a port.
pub fn port_write_string(i: &mut Interpreter) -> BuiltinRet {
    let str = i.stack_pop::<String>()?.into_inner();
    let mut port = i.stack_pop_val()?;
    let Some(p) = port.as_trait_mut::<dyn io::Write>() else {
        i.stack_push(port);
        i.stack_push_error(false);
        return Ok(());
    };
    let res = p.write(str.as_ref()).map(|usz| usz as i64).map_err(util::display);
    i.stack_push(port);
    i.stack_push_result(res);
    Ok(())
}

/// Read a specified range from a port into a bytevector.
/// `port bytevector start end port-read-range -> port bytevector read-count-or-error`
pub fn port_read_range(i: &mut Interpreter) -> BuiltinRet {
    let end = i.stack_pop::<i64>()?.into_inner();
    let start = i.stack_pop::<i64>()?.into_inner();
    let mut bytevector = i.stack_pop::<Vec<u8>>()?;
    let mut port = i.stack_pop_val()?;
    let Some(p) = port.as_trait_mut::<dyn io::Read>() else {
        i.stack_push(port);
        i.stack_push_error(false);
        return Ok(());
    };
    let bv = bytevector.as_mut();
    let range = util::bytes_range_mut(bv, start, end);
    let res = p.read(range).map(|c| c as i64).map_err(util::display);
    i.stack_push(port);
    i.stack_push(bytevector);
    i.stack_push_result(res);
    Ok(())
}

/// Write a specified range from a bytevector into a port.
/// Creates a builtin with the following signature:
/// `port bytevector start end port-write-bytevector-range -> port bytevector write-count-or-error`
/// See [bytes_range_mut] for da rulez.
pub fn port_write_range(i: &mut Interpreter) -> BuiltinRet {
    let end = i.stack_pop::<i64>()?.into_inner();
    let start = i.stack_pop::<i64>()?.into_inner();
    let bytevector = i.stack_pop::<Vec<u8>>()?;
    let mut port = i.stack_pop_val()?;
    let Some(p) = port.as_trait_mut::<dyn io::Write>() else {
        i.stack_push(port);
        i.stack_push_error(false);
        return Ok(());
    };
    let range = util::bytes_range(bytevector.as_ref(), start, end);
    let res = p.write(range).map(|c| c as i64).map_err(util::display);
    i.stack_push(port);
    i.stack_push(bytevector);
    i.stack_push_result(res);
    Ok(())
}

/// Flush an output port.
/// `port port-flush -> port true-or-error`
pub fn port_flush(i: &mut Interpreter) -> BuiltinRet {
    let mut port = i.stack_pop_val()?;
    let Some(p) = port.as_trait_mut::<dyn io::Write>() else {
        i.stack_push(port);
        i.stack_push_error(false);
        return Ok(());
    };
    let res = p.flush().map(|()| true).map_err(util::display);
    i.stack_push(port);
    i.stack_push_result(res);
    Ok(())
}

/// Install these port-related functions.
pub fn install(i: &mut Interpreter) {
    i.add_builtin("port-input?", port_input);
    i.add_builtin("port-output?", port_output);
    i.add_builtin("port-read->string", port_read_to_string);
    i.add_builtin("port-read-range", port_read_range);
    i.add_builtin("port-write-string", port_write_string);
    i.add_builtin("port-write-range", port_write_range);
    i.add_builtin("port-flush", port_flush);
}


