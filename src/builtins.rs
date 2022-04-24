
use crate::interpreter::Builder;

pub mod core;
pub mod define;
pub mod doc;
pub mod interpreter;
pub mod list;
pub mod module;
pub mod place;
pub mod port;
pub mod reader;
pub mod string;

pub fn install(i: Builder) -> Builder {
    let i = core::install(i);
    let i = define::install(i);
    let i = doc::install(i);
    let i = interpreter::install(i);
    let i = list::install(i);
    let i = module::install(i);
    let i = place::install(i);
    let i = port::install(i);
    let i = reader::install(i);
    let i = string::install(i);
    i
}

