
extern crate rustyline;
extern crate env_logger;
extern crate log;

extern crate hell;

use std::env;
use std::fmt;
use std::path::Path;
use hell::interpreter::*;
use rustyline::error::ReadlineError;

mod reader;

fn read_line(interpreter: &mut Interpreter, line: &str) {
    // TODO in repl mode, this will read the whole line before interpreting
    // which may be unexpected for e.g. load-file
    interpreter.push_input(line);
    interpreter.push_input("\n"); // I think
}

fn run<S: fmt::Debug + AsRef<str> + AsRef<Path>>(args: &[S]) {
    let mut interpreter = Interpreter::new(reader::default_reader());

    hell::stdlib::install(&mut interpreter);
 
    // TODO when removing REPL from this binary, just read from stdin or something
    if args.len() > 1 {
        let script = &args[1];

        if let Err(e) = interpreter.load_file(script.as_ref()) {
            eprintln!("Error loading script {:?}", script);
            eprintln!("{}", e);
        }
        if let Err(e) = interpreter.run_available() {
            eprintln!("Error in script {:?}", script);
            eprintln!("{}", e);
            eprintln!("Stack ({} items)", interpreter.stack.size());
            eprintln!("{}", interpreter.stack.describe());
            interpreter.clear();
        }
    }

    // interpreter.parser_mut().set_position(Source::new());

    run_repl(interpreter);
}

fn run_repl(mut interpreter: Interpreter) {
    let mut prompt;
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        /* Borrow parser to read unfinished tokens */ {
            let v = interpreter.unfinished();
            if v.len() > 0 {
                prompt = "(".to_string();
                prompt.push_str(&v.join(" "));
                prompt.push_str(")");
            } else {
                prompt = format!("{}", interpreter.stack.show());
            }
            if interpreter.quoting() {
                prompt.push_str(" ...");
            }
            prompt.push_str(" > ");
        }
        match rl.readline(prompt.as_str()) {
            Ok(line) => {
                rl.add_history_entry(&line);
                read_line(&mut interpreter, &line);
                if let Err(e) = interpreter.run_available() {
                    interpreter.clear();
                    eprintln!("{}", e);
                    continue;
                }
            },
            Err(ReadlineError::Interrupted) => {
                interpreter.clear();
            },
            Err(ReadlineError::Eof) => {
                break;
            },
            e => {
                eprintln!("Read error: {:?}", e);
                break;
            },
        }
    }

}

fn main() {

    env_logger::Builder::from_default_env()
        .default_format_timestamp_nanos(true)
        .init();

    let args: Vec<String> = env::args().collect();

    run(&args[..]);
}

