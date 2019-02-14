
use crate::data::*;
use crate::interpreter::code::*;
use crate::interpreter::exec;
use crate::interpreter::Interpreter;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.add_builtin("uplevel", uplevel);
    interpreter.add_builtin("quote", quote);
    interpreter.add_builtin("list->definition", list_into_definition);
    interpreter.add_builtin("get-definition", get_definition);
    interpreter.add_builtin("take-definition", take_definition);
    interpreter.add_builtin("resolve-definition", resolve_definition);
    interpreter.add_builtin("add-definition", add_definition);
    interpreter.add_builtin("eval-definition", eval_definition);
    interpreter.add_builtin("defined?", is_defined);
    interpreter.define_type_predicate::<Code>("definition?");
    interpreter.add_builtin("defined-names", defined_names);
    interpreter.add_builtin("call", call);
    interpreter.add_builtin("call-when", call_when);
    interpreter.add_builtin("read-eval-file", read_eval_file);
    interpreter.add_builtin("uplevel-in-named-context", uplevel_in_named_context);
    interpreter.add_builtin("abort", abort);
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

fn list_into_definition(interpreter: &mut Interpreter) -> exec::Result<()> {
    let code = interpreter.stack.pop::<List>()?;
    let def = Code::from(Definition::new(code.into()));
    interpreter.stack.push(Datum::new(def));
    Ok(())
}

fn get_definition(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    match interpreter.env_mut().get_definition(&name) {
        Some(def) => {
            interpreter.stack.push(Datum::new(def));
        },
        None => Err(error::NotDefined(name))?,
    }
    Ok(())
}

fn take_definition(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    match interpreter.env_mut().undefine(&name) {
        Some(def) => {
            interpreter.stack.push(Datum::new(def));
        },
        None => Err(error::NotDefined(name))?,
    }
    Ok(())
}

fn resolve_definition(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    match interpreter.context.resolve(&name) {
        Some(def) => {
            interpreter.stack.push(Datum::new(def.clone()));
        },
        None => Err(error::NotDefined(name))?,
    }
    Ok(())
}

fn add_definition(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    let def = interpreter.stack.pop::<Code>()?;
    interpreter.env_mut().define(name, def);
    Ok(())
}

fn eval_definition(interpreter: &mut Interpreter) -> exec::Result<()> {
    let code = interpreter.stack.pop::<Code>()?;
    interpreter.eval_code(&code)?;
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

fn read_eval_file(interpreter: &mut Interpreter) -> exec::Result<()> {
    let file = interpreter.stack.pop::<String>()?;
    interpreter.eval_file(&file)?;
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
    Ok(Err(error::Abort())?)
}

