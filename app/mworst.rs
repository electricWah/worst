
// extern crate env_logger;
// extern crate simple_logger;

extern crate hell;

use std::env;
use std::fmt;
use std::path::Path;
use hell::interpreter::*;

fn run<S: fmt::Debug + AsRef<str> + AsRef<Path>>(args: &[S]) {
    let mut interpreter = Interpreter::new();

    hell::stdlib::install(&mut interpreter);
 
    // TODO when removing REPL from this binary, just read from stdin or something
    if args.len() > 1 {
        let script = &args[1];

        if let Err(e) = interpreter.eval_file(script.as_ref()) {
            eprintln!("Error: {}", e);
            eprintln!("History:");
            for x in interpreter.history() {
                eprintln!("  {}", x.as_str());
            }
            if interpreter.stack.size() > 0 {
                eprintln!("Stack ({} items)", interpreter.stack.size());
                eprintln!("{}", interpreter.stack.describe());
            }
            interpreter.clear();
        }
    }

    // interpreter.parser_mut().set_position(Source::new());

    // run_repl(interpreter);
}

fn main() {

    // env_logger::Builder::from_default_env()
    //     .default_format_timestamp_nanos(true)
    //     .init();

    // simple_logger::init_with_level(log::Level::Error).unwrap();

    let args: Vec<String> = env::args().collect();

    run(&args[..]);
}

