
//! Querying and interacting with the ambient operating system

use crate::list::List;
use crate::interpreter::{Interpreter, Handle};

/// `command-line-arguments` -> list :
/// the command-line arguments from program invocation.
pub async fn command_line_arguments(mut i: Handle) {
    i.stack_push(List::from_iter(std::env::args())).await;
}

/// string `get-environment-variable` +-> string|false :
/// the value of the environment variable. See [std::env::var].
pub async fn get_environment_variable(mut i: Handle) {
    match std::env::var(i.stack_top::<String>().await.as_ref()) {
        Ok(v) => i.stack_push(v).await,
        Err(_) => i.stack_push(false).await,
    }
}

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.define("command-line-arguments", command_line_arguments);
    i.define("get-environment-variable", get_environment_variable);
}
