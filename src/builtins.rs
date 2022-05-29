
use crate::interpreter::Interpreter;

pub mod core;
pub mod define;
pub mod doc;
pub mod fs;
pub mod interpreter;
pub mod io;
pub mod list;
pub mod module;
pub mod place;
pub mod reader;
pub mod string;

pub fn install(i: &mut Interpreter) {
    core::install(i);
    define::install(i);
    doc::install(i);
    fs::install(i);
    interpreter::install(i);
    io::install(i);
    list::install(i);
    module::install(i);
    place::install(i);
    reader::install(i);
    string::install(i);
}

