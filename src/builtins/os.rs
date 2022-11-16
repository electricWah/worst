
//! Querying and interacting with the ambient operating system

use crate::base::*;
use crate::interpreter::{Interpreter, Handle};

/// `command-line-arguments` -> list :
/// the command-line arguments from program invocation.
pub async fn command_line_arguments(mut i: Handle) {
    i.stack_push(List::from_iter(std::env::args())).await;
}

/// string `environment-variable` -> string|false :
/// the value of the environment variable. See [std::env::var].
pub async fn environment_variable(mut i: Handle) {
    let v = i.stack_pop::<String>().await;
    i.stack_push_option(std::env::var(v.as_ref()).ok()).await;
}

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.define("command-line-arguments", command_line_arguments);
    i.define("environment-variable", environment_variable);
}

