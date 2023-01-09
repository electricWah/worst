
use std::process::ExitCode;
use worst::interp2::*;
use worst::builtins;
use worst::base::*;

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
        eprint!("(");
        for v in v.iter() {
            basic_printerr(v);
            eprint!(" ");
        }
        eprint!(")");
    } else {
        eprint!("(value)");
    }
}

fn main() -> ExitCode {
    let init_module = std::env::var("WORST_INIT_MODULE").unwrap_or_else(|_| "worst/init".into());
    let mut i = Interpreter::new(List::from_iter(vec!["import".to_symbol(), init_module.to_symbol()].into_iter()));
    builtins::install(&mut i);
    if let Err(e) = i.run() {
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

