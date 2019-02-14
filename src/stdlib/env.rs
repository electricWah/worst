
use std::env;
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;

use crate::stdlib::hashtable::HashTable;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.add_builtin("command-line", command_line);
    interpreter.add_builtin("get-environment-variable", get_environment_variable);
    interpreter.add_builtin("set-environment-variable", set_environment_variable);
    interpreter.add_builtin("get-environment-variables", get_environment_variables);
}

fn command_line(interpreter: &mut Interpreter) -> exec::Result<()> {
    let args: Vec<String> = env::args().collect();
    interpreter.stack.push(Datum::new(List::from(args)));
    Ok(())
}

fn get_environment_variable(interpreter: &mut Interpreter) -> exec::Result<()> {
    let var = interpreter.stack.pop::<String>()?;
    let res = env::var(var).ok();
    match res {
        Some(r) => interpreter.stack.push(Datum::new(r)),
        None => interpreter.stack.push(Datum::new(false)),
    }
    Ok(())
}

fn set_environment_variable(interpreter: &mut Interpreter) -> exec::Result<()> {
    let v = interpreter.stack.pop::<String>()?;
    let k = interpreter.stack.pop::<String>()?;
    env::set_var(k, v);
    Ok(())
}

fn get_environment_variables(interpreter: &mut Interpreter) -> exec::Result<()> {
    let mut tbl = HashTable::default();
    env::vars().for_each(
        |(k, v)| tbl.set(Datum::new(k),
        Datum::new(v)));
    interpreter.stack.push(Datum::new(tbl));
    Ok(())
}

