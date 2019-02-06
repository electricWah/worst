
// pub mod enumcommand;

pub mod bool;
// pub mod combo;
pub mod control;
pub mod datum;
pub mod env;
pub mod file;
pub mod hashtable;
// pub mod interpreter;
pub mod list;
// pub mod net;
pub mod number;
// pub mod parsing;
pub mod place;
pub mod port;
pub mod process;
// pub mod record;
pub mod stack;
pub mod string;
pub mod vector;

use crate::interpreter::Interpreter;

pub fn install(interpreter: &mut Interpreter) {
    bool::install(interpreter);
    // combo::install(interpreter);
    control::install(interpreter);
    datum::install(interpreter);
    env::install(interpreter);
    file::install(interpreter);
    hashtable::install(interpreter);
    // interpreter::install(interpreter);
    list::install(interpreter);
    // net::install(interpreter);
    number::install(interpreter);
    // parsing::install(interpreter);
    place::install(interpreter);
    port::install(interpreter);
    process::install(interpreter);
    // record::install(interpreter);
    stack::install(interpreter);
    string::install(interpreter);
    vector::install(interpreter);
}

