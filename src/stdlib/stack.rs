
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.add_builtin("clone", op_clone);
    interpreter.add_builtin("dig", op_dig);
    interpreter.add_builtin("drop", op_drop);
    interpreter.add_builtin("stack-empty?", is_stack_empty);
}

fn op_clone(interpreter: &mut Interpreter) -> exec::Result<()> {
    let d = { interpreter.stack.ref_datum(0)?.clone() };
    interpreter.stack.push(d);
    Ok(())
}

fn op_dig(interpreter: &mut Interpreter) -> exec::Result<()> {
    let n = interpreter.stack.pop::<isize>()?;
    if n > 0 {
        let n = n as usize;
        let a = interpreter.stack.remove(n)?;
        interpreter.stack.push(a);
    } else if n < 0 {
        let n = -n as usize;
        let a = interpreter.stack.pop_datum()?;
        interpreter.stack.insert(a, n - 1)?;
    }
    Ok(())
}

fn op_drop(interpreter: &mut Interpreter) -> exec::Result<()> {
    interpreter.stack.pop_datum()?;
    Ok(())
}

fn is_stack_empty(interpreter: &mut Interpreter) -> exec::Result<()> {
    let size = interpreter.stack.size();
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(size == 0));
    Ok(())
}


