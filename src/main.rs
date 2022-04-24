
use std::path::PathBuf;
use std::io::Read;
use clap::Parser;

use worst::interpreter::*;
use worst::builtins;
use worst::base::*;
use worst::list::List;
use worst::reader::read_all;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Worst source file to evaluate
    #[clap(parse(from_os_str))]
    file: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut f = std::fs::File::open(args.file)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let body = List::from(read_all(&mut s.chars()).unwrap_or_else(|e| {
        println!("{:?}", e);
        vec![]
    }));

    let mut i = builtins::install(Builder::default()).eval(Val::from(List::from(body)));
    if !i.run() {
        while let Some(sp) = i.stack_pop_val() {
            println!("{:?}", sp);
        }
    }
    Ok(())
}

