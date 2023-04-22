
use std::process::ExitCode;
use worst::builtins;
use worst::base::*;

fn basic_printerr(v: &Val) {
    if let Some(v) = v.downcast_ref::<Symbol>() {
        eprint!("{v}");
    } else if let Some(v) = v.downcast_ref::<bool>() {
        eprint!("{}", if *v { "#t" } else { "#f" });
    } else if let Some(v) = v.downcast_ref::<String>() {
        eprint!("{v:?}");
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
    let mut i = worst::embedded();
    builtins::install(&mut i);
    if let Err(e) = i.run() {
        // if IsError::is_error(&e) {
        //     eprint!("\nTop-level error: ");
        // }
        basic_printerr(&e);
        eprint!("\nStack: ");
        for v in i.stack_ref().iter() {
            basic_printerr(v);
            eprint!(" ");
        }
        eprintln!("\nCall stack:");
        for name in i.call_stack_names() {
            eprintln!("  {}", name.unwrap_or("???".to_string()));
        }
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

