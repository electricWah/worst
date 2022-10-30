
use std::process::ExitCode;
use worst::interpreter::*;
use worst::builtins;
use worst::base::*;
use worst::list::List;

fn basic_printerr(v: &Val) {
    if let Some(v) = v.downcast_ref::<Symbol>() {
        eprint!("{v}");
    } else if let Some(v) = v.downcast_ref::<String>() {
        eprint!("{v}");
    } else if let Some(v) = v.downcast_ref::<i64>() {
        eprint!("{v}");
    } else if let Some(v) = v.downcast_ref::<f64>() {
        eprint!("{v}");
    } else if let Some(v) = v.downcast_ref::<List>() {
        for v in v.iter() {
            basic_printerr(v);
            eprint!(" ");
        }
    } else {
        eprint!("(value)");
    }
}

fn main() -> ExitCode {
    let init_module = std::env::var("WORST_INIT_MODULE").unwrap_or_else(|_| "worst/init".into());
    let mut i = Interpreter::default();
    builtins::install(&mut i);
    let doit = vec!["import".into(), init_module].into_iter().map(Symbol::from);
    i.eval_next(Val::from(List::from_iter(doit)));
    if let Some(e) = i.run() {
        if IsError::is_error(&e) {
            eprint!("\nTop-level error: ");
            basic_printerr(&e);
            eprintln!();
            eprint!("\nStack: ");
            for v in i.stack_ref().iter() {
                basic_printerr(v);
            }
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}

