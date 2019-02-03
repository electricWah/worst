
use std::mem;
use crate::data::*;
use crate::interpreter::*;
use crate::interpreter::exec;
use crate::interpreter::code::*;

use super::data::*;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<InterpRef>("interpreter?");

    interpreter.add_builtin("current-interpreter", current_interpreter);
    interpreter.add_builtin("make-interpreter", make_interpreter);
    interpreter.add_builtin("interpreter-clear", interpreter_clear);
    interpreter.add_builtin("interpreter-read-next", interpreter_read_next);
    interpreter.add_builtin("interpreter-read-file", interpreter_read_file);
    interpreter.add_builtin("interpreter-swap-stack", interpreter_swap_stack);
    interpreter.add_builtin("interpreter-add-definition", interpreter_add_definition);
    interpreter.add_builtin("interpreter-get-definition", interpreter_get_definition);
    interpreter.add_builtin("interpreter-take-definition", interpreter_take_definition);
    interpreter.add_builtin("interpreter-resolve-symbol", interpreter_resolve_symbol);
    interpreter.add_builtin("interpreter-eval-code", interpreter_eval_code);
    interpreter.add_builtin("interpreter-root-context?", is_interpreter_root_context);
    interpreter.add_builtin("interpreter-set-context-name", interpreter_set_context_name);
    interpreter.add_builtin("interpreter-context-name", interpreter_context_name);
    interpreter.add_builtin("interpreter-quoting?", is_interpreter_quoting);
    interpreter.add_builtin("current-interpreter?", is_current_interpreter);

}

fn current_interpreter(interpreter: &mut Interpreter) -> exec::Result<()> {
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(InterpRef::current()));
    Ok(())
}

fn make_interpreter(interpreter: &mut Interpreter) -> exec::Result<()> {
    let interp = InterpRef::from(Interpreter::new());
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(interp));
    Ok(())
}

fn interpreter_clear(interpreter: &mut Interpreter) -> exec::Result<()> {
    with_top_mut(interpreter, |i| {
        i.clear();
    })?;
    Ok(())
}

fn interpreter_read_next(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = with_top_mut(interpreter, |i| {
        i.read_next()
    })?;
    match r {
        Ok(None) => {
            interpreter.stack.push(Datum::new(false));
            interpreter.stack.push(Datum::new(true));
        },
        Ok(Some(res)) => {
            interpreter.stack.push(res);
            interpreter.stack.push(Datum::new(true));
        },
        Err(e) => {
            interpreter.stack.push(Datum::new(e));
            interpreter.stack.push(Datum::new(false));
        },
    }
    Ok(())
}

fn interpreter_read_file(interpreter: &mut Interpreter) -> exec::Result<()> {
    let file = interpreter.stack.pop::<String>()?;
    let r = with_top_mut(interpreter, |i| {
        i.eval_file(&file)
    })?;
    match r {
        Ok(()) => {
            interpreter.stack.push(Datum::new(true));
        },
        Err(e) => {
            interpreter.stack.push(Datum::new(e));
            interpreter.stack.push(Datum::new(false));
        },
    }
    Ok(())
}

fn interpreter_swap_stack(interpreter: &mut Interpreter) -> exec::Result<()> {
    let mut l = interpreter.stack.pop::<List>()?.into();
    with_top_mut(interpreter, |i| {
        mem::swap(i.stack.vec_data_mut(), &mut l);
    })?;
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(List::from(l)));
    Ok(())
}

fn interpreter_add_definition(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    let def = interpreter.stack.pop::<Code>()?;
    with_top_mut(interpreter, |i| {
        i.context.define(name, def)
    })?;
    Ok(())
}

fn interpreter_get_definition(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    let r = with_top_mut(interpreter, |i| {
        i.context.get_definition(&name)
    })?;

    interpreter.stack.push(r.map_or(Datum::new(false),
    |v| Datum::build().with_source(v.source()).ok(v)));
    Ok(())
}

fn interpreter_take_definition(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    let r = with_top_mut(interpreter, |i| {
        i.context.undefine(&name)
    })?;

    interpreter.stack.push(r.map_or(Datum::new(false),
    |v| Datum::build().with_source(v.source()).ok(v)));
    Ok(())
}

fn interpreter_resolve_symbol(interpreter: &mut Interpreter) -> exec::Result<()> {
    let sym = interpreter.stack.pop::<Symbol>()?;
    let r = with_top_mut(interpreter, |i| {
        i.resolve_symbol(&sym)
    })?;
    match r {
        Some(r) => interpreter.stack.push(Datum::new(r)),
        None => interpreter.stack.push(Datum::new(false)),
    }
    Ok(())
}

fn interpreter_eval_code(interpreter: &mut Interpreter) -> exec::Result<()> {
    let (code, src) = interpreter.stack.pop_source::<Code>()?;
    let r = with_top_mut(interpreter, |i| {
        i.eval_code(&code, src)
    })?;
    match r {
        Ok(()) => {
            interpreter.stack.push(Datum::new(true));
        },
        Err(e) => {
            interpreter.stack.push(Datum::new(e));
            interpreter.stack.push(Datum::new(false));
        },
    }
    Ok(())
}

fn is_interpreter_root_context(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = with_top_mut(interpreter, |i| {
        i.context.is_root()
    })?;
    interpreter.stack.push(Datum::new(r));
    Ok(())
}

fn interpreter_set_context_name(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    with_top_mut(interpreter, |i| {
        i.context.set_name(Some(name.to_string()));
    })?;
    Ok(())
}

fn interpreter_context_name(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = with_top_mut(interpreter, |i| {
        i.context.name().map(Symbol::from)
    })?;
    match name {
        Some(n) => interpreter.stack.push(Datum::new(n)),
        None => interpreter.stack.push(Datum::new(false)),
    }
    Ok(())
}

fn is_interpreter_quoting(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = with_top_mut(interpreter, |i| {
        i.quoting()
    })?;
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(r));
    Ok(())
}

fn is_current_interpreter(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = interpreter.stack.ref_at::<InterpRef>(0)? == &InterpRef::Current;
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(r));
    Ok(())
}

fn with_top_mut<T, F: FnOnce(&mut Interpreter) -> T>(i: &mut Interpreter, f: F) -> exec::Result<T> {
    let (mut interp, src) = i.stack.pop_source::<InterpRef>()?;
    let r = match interp {
        InterpRef::Current => f(i),
        InterpRef::Ref(ref mut ii) => f(&mut ii.borrow_mut()),
    };
    i.stack.push(Datum::build().with_source(src).ok(interp));
    Ok(r)
}

