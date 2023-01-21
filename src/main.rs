
use std::process::ExitCode;
use worst::interp2::*;
use worst::builtins;
use worst::base::*;
use worst::reader;

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

static WORST_INIT: &str = include_str!("main.w");

fn main() -> ExitCode {
    let wmain =
        match reader::read_all(&mut WORST_INIT.chars()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{e:?}");
                return ExitCode::FAILURE;
            },
        };
    let mut i = Interpreter::new(wmain);
    builtins::install(&mut i);
    if let Err(e) = i.run() {
        if IsError::is_error(&e) {
            eprint!("\nTop-level error: ");
        }
        basic_printerr(&e);
        eprintln!();
        eprint!("\nStack: ");
        for v in i.stack_ref().iter() {
            basic_printerr(v);
            eprint!(" ");
        }
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

