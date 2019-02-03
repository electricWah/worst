
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<List>("list?");
    interpreter.add_builtin("list-push-head", list_push_head);
    interpreter.add_builtin("list-push-tail", list_push_tail);
    interpreter.add_builtin("list-pop-head", list_pop_head);
    interpreter.add_builtin("list-pop-tail", list_pop_tail);
    interpreter.add_builtin("list-append", list_append);
    interpreter.add_builtin("list-length", list_length);
    interpreter.add_builtin("list-swap", list_swap);
}

fn list_push_head(interpreter: &mut Interpreter) -> exec::Result<()> {
    let a = interpreter.stack.pop_datum()?;
    let l = interpreter.stack.top_mut::<List>()?;
    l.push_head(a);
    Ok(())
}

fn list_push_tail(interpreter: &mut Interpreter) -> exec::Result<()> {
    let a = interpreter.stack.pop_datum()?;
    let l = interpreter.stack.top_mut::<List>()?;
    l.push_tail(a);
    Ok(())
}

fn list_pop_head(interpreter: &mut Interpreter) -> exec::Result<()> {
    let a = {
        let l = interpreter.stack.top_mut::<List>()?;
        l.pop_head().ok_or(error::ListEmpty())?
    };
    interpreter.stack.push(a);
    Ok(())
}

fn list_pop_tail(interpreter: &mut Interpreter) -> exec::Result<()> {
    let a = {
        let l = interpreter.stack.top_mut::<List>()?;
        l.pop_tail().ok_or(error::ListEmpty())?
    };
    interpreter.stack.push(a);
    Ok(())
}

fn list_append(interpreter: &mut Interpreter) -> exec::Result<()> {
    let b = interpreter.stack.pop::<List>()?;
    let a = interpreter.stack.top_mut::<List>()?;
    a.append(b);
    Ok(())
}

fn list_length(interpreter: &mut Interpreter) -> exec::Result<()> {
    let len = { interpreter.stack.ref_at::<List>(0)?.len() };
    interpreter.stack.push(Datum::build().with_source(interpreter.current_source()).ok(len as isize));
    Ok(())
}

fn list_swap(interpreter: &mut Interpreter) -> exec::Result<()> {
    let j = interpreter.stack.pop::<isize>()?.cast::<usize>()?;
    let i = interpreter.stack.pop::<isize>()?.cast::<usize>()?;
    let lis = interpreter.stack.top_mut::<List>()?;
    let len = lis.len();
    if i > len {
        Err(error::OutOfRange(0, len as isize - 1, i as isize))?;
    }
    if j > len {
        Err(error::OutOfRange(0, len as isize - 1, j as isize))?;
    }
    lis.swap(i, j);
    Ok(())
}

