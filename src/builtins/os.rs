
//! Querying and interacting with the ambient operating system

use crate::base::*;
use crate::interp2::*;

/// `command-line-arguments` -> list :
/// the command-line arguments from program invocation.
pub fn command_line_arguments(i: &mut Interpreter) -> BuiltinRet {
    i.stack_push(List::from_iter(std::env::args()));
    Ok(())
}

/// string `environment-variable` -> string|false :
/// the value of the environment variable. See [std::env::var].
pub fn environment_variable(i: &mut Interpreter) -> BuiltinRet {
    let v = i.stack_pop::<String>()?;
    i.stack_push_option(std::env::var(v.as_ref()).ok());
    Ok(())
}

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.add_builtin("command-line-arguments", command_line_arguments);
    i.add_builtin("environment-variable", environment_variable);
}

