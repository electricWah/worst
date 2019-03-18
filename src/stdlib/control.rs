
use crate::data::*;
use crate::interpreter::exec;
use crate::interpreter::Interpreter;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.add_builtin("uplevel", uplevel);
    interpreter.add_builtin("quote", quote);
    interpreter.add_builtin("take-definition", take_definition);
    interpreter.add_builtin("resolve-definition", resolve_definition);
    interpreter.add_builtin("add-definition", add_definition);
    interpreter.add_builtin("eval-definition", eval_definition);
    interpreter.add_builtin("defined?", is_defined);
    interpreter.add_builtin("defined-names", defined_names);
    interpreter.add_builtin("eval-builtin", eval_builtin);
    interpreter.add_builtin("call", call);
    interpreter.add_builtin("call-when", call_when);
    interpreter.add_builtin("read-file", read_file);
    interpreter.add_builtin("uplevel-in-named-context", uplevel_in_named_context);
    interpreter.add_builtin("abort", abort);
    interpreter.add_builtin("interpreter-clear", interreter_clear);
}

fn uplevel(interpreter: &mut Interpreter) -> exec::Result<()> {
    interpreter.context.uplevel()?;
    let name = interpreter.stack.pop::<Symbol>()?;
    interpreter.eval_symbol(&name)?;
    Ok(())
}

fn quote(interpreter: &mut Interpreter) -> exec::Result<()> {
    interpreter.quote_next();
    Ok(())
}

fn take_definition(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    match interpreter.env_mut().undefine(&name) {
        Some(def) => {
            interpreter.stack.push(Datum::new(List::from(def)));
        },
        None => Err(error::NotDefined(name))?,
    }
    Ok(())
}

fn resolve_definition(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    match interpreter.context.resolve(&name) {
        Some(def) => {
            interpreter.stack.push(Datum::new(List::from_iter(def.iter())));
        },
        None => {
            interpreter.stack.push(Datum::new(false));
        },
    }
    Ok(())
}

fn add_definition(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    let def = interpreter.stack.pop::<List>()?;
    interpreter.env_mut().define(name, def);
    Ok(())
}

fn eval_definition(interpreter: &mut Interpreter) -> exec::Result<()> {
    let def = interpreter.stack.pop::<List>()?;
    interpreter.eval_definition(def)?;
    // interpreter.stack.push(code);
    Ok(())
}

fn is_defined(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = {
        let name = interpreter.stack.ref_at::<Symbol>(0)?;
        interpreter.env().is_defined(name)
    };
    interpreter.stack.push(Datum::new(r));
    Ok(())
}

fn defined_names(interpreter: &mut Interpreter) -> exec::Result<()> {
    // TODO source
    let names: Vec<Datum> = interpreter.env_mut().current_defines()
        .map(Clone::clone)
        .map(|s| Datum::symbol(s))
        .collect();
    interpreter.stack.push(Datum::new::<List>(names.into()));
    Ok(())
}

fn eval_builtin(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    interpreter.eval_builtin(&name)?;
    Ok(())
}

fn call(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    interpreter.eval_symbol(&name)?;
    Ok(())
}

fn call_when(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    let whether = interpreter.stack.pop::<bool>()?;
    if whether {
        return interpreter.eval_symbol(&name);
    }
    Ok(())
}

fn read_file(interpreter: &mut Interpreter) -> exec::Result<()> {
    let file = interpreter.stack.pop::<String>()?;
    let l = List::from(Interpreter::read_file(&file)?);
    interpreter.stack.push(Datum::new(l));
    Ok(())
}

fn uplevel_in_named_context(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    let sym = interpreter.stack.pop::<Symbol>()?;
    while interpreter.context.name() != Some(name.as_ref()) {
        interpreter.context.uplevel()?;
    }
    interpreter.eval_symbol(&sym)?;
    Ok(())
}

fn abort(_interpreter: &mut Interpreter) -> exec::Result<()> {
    Ok(Err(error::Abort)?)
}

fn interreter_clear(interpreter: &mut Interpreter) -> exec::Result<()> {
    interpreter.clear();
    Ok(())
}

