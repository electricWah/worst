
use std::process::ExitCode;
use worst::interpreter::*;
use worst::builtins;
use worst::base::*;
use worst::list::List;

fn main() -> ExitCode {
    let init_module = std::env::var("WORST_INIT_MODULE").unwrap_or_else(|_| "worst/init".into());
    let mut i = Interpreter::default();
    builtins::install(&mut i);
    let doit = vec!["import".into(), init_module].into_iter().map(Symbol::from);
    i.eval_next(Val::from(List::from_iter(doit)));
    if let Some(e) = i.run() {
        if IsError::is_error(&e) {
            println!("{:?}", e);
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}

