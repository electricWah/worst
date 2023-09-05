
use std::fmt;
use std::process::ExitCode;
use worst::builtins;
use worst::base::*;

fn main() -> ExitCode {
    let mut i = worst::embedded();
    builtins::install(&mut i);
    if let Err(e) = i.run() {
        if let Some(d) = e.as_trait_ref::<dyn fmt::Display>() {
            eprint!("{}", d);
        } else {
            eprint!("interpreter error");
        }
        eprint!("\nStack: ");
        eprint!("{}", i.stack_ref());
        eprintln!("\nCall stack:");
        let u = i.uniques_mut().get_type::<Symbol>();
        let stack = i.stack_meta_refs().map(|m| m.get_val(&u));
        for name in stack {
            if let Some(name) = name {
                eprintln!("  {}", name.downcast_ref::<Symbol>().unwrap());
            } else {
                eprintln!("  ???");
            }
        }
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

