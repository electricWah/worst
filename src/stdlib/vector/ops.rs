
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;
use crate::stdlib::vector::data::*;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<U8Vector>("u8vector?");
    interpreter.add_builtin("make-u8vector", make_u8vector);
    interpreter.add_builtin("u8vector-length", u8vector_length);
    interpreter.add_builtin("u8vector-get", u8vector_get);
    interpreter.add_builtin("u8vector-set", u8vector_set);
    // interpreter.add_builtin("u8vector-split", u8vector_split);
    interpreter.add_builtin("u8vector-truncate", u8vector_truncate);
    interpreter.add_builtin("u8vector-extend", u8vector_extend);
    interpreter.add_builtin("u8vector-append", u8vector_append);
    interpreter.add_builtin("u8vector-push", u8vector_push);
    // interpreter.add_builtin("u8vector->list", u8vector_into_list);
    // interpreter.add_builtin("list->u8vector", list_into_u8vector);
}

fn make_u8vector(interpreter: &mut Interpreter) -> exec::Result<()> {
    let len = interpreter.stack.pop::<isize>()?.cast::<usize>()?;
    let fill = interpreter.stack.pop::<isize>()?.cast::<u8>()?;
    let vec = U8Vector::fill(len, fill);
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(vec));
    Ok(())
}

fn u8vector_length(interpreter: &mut Interpreter) -> exec::Result<()> {
    let len = interpreter.stack.ref_at::<U8Vector>(0)?.len();
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(isize::from_num(len)?));
    Ok(())
}

fn u8vector_get(interpreter: &mut Interpreter) -> exec::Result<()> {
    let idx = interpreter.stack.pop::<isize>()?.cast::<usize>()?;
    let (len, got) = {
        let vec = interpreter.stack.ref_at::<U8Vector>(0)?;
        (vec.len(), vec.inner().get(idx).cloned())
    };
    match got {
        Some(v) => {
            let source = interpreter.current_source();
            interpreter.stack.push(Datum::build().with_source(source).ok(isize::from_num(v)?));
        },
        None => return Err(error::OutOfRange(0, len as isize, idx as isize).into()),
    }
    Ok(())
}

fn u8vector_set(interpreter: &mut Interpreter) -> exec::Result<()> {
    let idx = interpreter.stack.pop::<isize>()?.cast::<usize>()?;
    let val = interpreter.stack.pop::<isize>()?.cast::<u8>()?;
    let bad = {
        let vec = interpreter.stack.top_mut::<U8Vector>()?.inner_mut();
        let len = vec.len();
        if let Some(x) = vec.get_mut(idx) {
            *x = val;
            None
        } else {
            Some(len)
        }
    };
    if let Some(len) = bad {
        return Err(error::OutOfRange(0, (len - 1) as isize, idx as isize).into());
    }
    Ok(())
}

// fn u8vector_split(interpreter: &mut Interpreter) -> exec::Result<()> {
//     Err(error::NotImplemented().into())
// }

fn u8vector_truncate(interpreter: &mut Interpreter) -> exec::Result<()> {
    let len = interpreter.stack.pop::<isize>()?.cast::<usize>()?;
    let vec = interpreter.stack.top_mut::<U8Vector>()?;
    vec.inner_mut().truncate(len);
    vec.inner_mut().shrink_to_fit();
    Ok(())
}

fn u8vector_extend(interpreter: &mut Interpreter) -> exec::Result<()> {
    let len = interpreter.stack.pop::<isize>()?.cast::<usize>()?;
    let val = interpreter.stack.pop::<isize>()?.cast::<u8>()?;
    let vec = interpreter.stack.top_mut::<U8Vector>()?;
    let clen = vec.len();
    vec.inner_mut().resize(len + clen, val);
    Ok(())
}

fn u8vector_append(interpreter: &mut Interpreter) -> exec::Result<()> {
    let mut b = interpreter.stack.pop::<U8Vector>()?;
    let a = interpreter.stack.top_mut::<U8Vector>()?;
    a.inner_mut().append(b.inner_mut());
    Ok(())
}

fn u8vector_push(interpreter: &mut Interpreter) -> exec::Result<()> {
    let v = interpreter.stack.pop::<isize>()?.cast::<u8>()?;
    let vec = interpreter.stack.top_mut::<U8Vector>()?;
    vec.inner_mut().push(v);
    Ok(())
}

// fn u8vector_into_list(interpreter: &mut Interpreter) -> exec::Result<()> {
//     Err(error::NotImplemented().into())
// }

// fn list_into_u8vector(interpreter: &mut Interpreter) -> exec::Result<()> {
//     Err(error::NotImplemented().into())
// }

