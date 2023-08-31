
//! A bytevector is a bunch of bytes (a [Vec<u8>]).
//! Use it to concatenate a lot of strings,
//! read or write data and such,
//! add (line-)buffering to an input port,
//! and whatever else it is you want to do with a bunch of bytes.

use crate::builtins::util;
use crate::interpreter::*;

/// Install some bytevector definitions.
pub fn install(i: &mut Interpreter) {

    util::add_const_type_builtin::<Vec<u8>>(i, "<bytevector>");
    i.add_builtin("bytevector-equal", util::equality::<Vec<u8>>);
    i.add_builtin("bytevector-length", |i: &mut Interpreter| {
        let v = i.stack_top::<Vec<u8>>()?;
        i.stack_push(v.as_ref().len() as i64);
        Ok(())
    });
    // ??? bytevector/string/i8/u8/i32/f32/...
    // i.add_builtin("bytevector-get", |i: &mut Interpreter| {
    // });
    // i.add_builtin("bytevector-set", |i: &mut Interpreter| {
    // });

    // bv start len bytevector-range -> bv
    // combination truncate + extend + substring
    // if start < 0, take from end
    // if start > 0, remove < start bytes
    // if len goes beyond end, pad with zeroes
    i.add_builtin("bytevector-range", |i: &mut Interpreter| {
        let end = i.stack_pop::<i64>()?.into_inner();
        let start = i.stack_pop::<i64>()?.into_inner();
        let mut v = i.stack_pop::<Vec<u8>>()?;
        let (start, end) = util::get_range(v.as_ref(), start, end, true);
        if start == end {
            (*v.as_mut()) = vec![];
        } else {
            let vmut = v.as_mut();
            let mut newv = vmut.split_off(start);
            std::mem::swap(vmut, &mut newv);
        }
        v.as_mut().resize(end - start, 0);
        i.stack_push(v);
        Ok(())
    });

    i.add_builtin("bytevector-split", |i: &mut Interpreter| {
        let idx = i.stack_pop::<i64>()?.into_inner();
        let mut a = i.stack_pop::<Vec<u8>>()?;
        let idx = util::index_range(a.as_ref().len(), idx, false);
        let b = a.as_mut().split_off(idx);
        i.stack_push(b);
        i.stack_push(a);
        Ok(())
    });

    i.add_builtin("string-utf8->bytevector", |i: &mut Interpreter| {
        let s = i.stack_pop::<String>()?.into_inner();
        i.stack_push(Vec::<u8>::from(s.as_str()));
        Ok(())
    });
    i.add_builtin("bytevector->string-utf8", |i: &mut Interpreter| {
        let bv = i.stack_pop::<Vec<u8>>()?.into_inner();
        match String::from_utf8(bv) {
            Ok(s) => i.stack_push(s),
            Err(e) => i.stack_push_error(format!("{}", e)),
        }
        Ok(())
    });
}

