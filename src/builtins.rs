
//! Core and auxiliary Worst functions implemented in Rust.

use crate::interpreter::Interpreter;

pub mod util;

pub mod bytevector;
pub mod core;
pub mod define;
pub mod dynamic;
pub mod doc;
#[cfg(feature = "builtin_file_module")]
pub mod file;
pub mod interpreter;
pub mod io;
pub mod list;
pub mod module;
pub mod numeric;
#[cfg(feature = "enable_os")]
pub mod os;
pub mod place;
pub mod reader;
pub mod string;

/// Define all enabled builtins in the given [Interpreter].
pub fn install(i: &mut Interpreter) {
    bytevector::install(i);
    core::install(i);
    define::install(i);
    dynamic::install(i);
    doc::install(i);
    #[cfg(feature = "builtin_file_module")]
    file::install(i);
    interpreter::install(i);
    io::install(i);
    list::install(i);
    module::install(i);
    numeric::install(i);
    #[cfg(feature = "enable_os")]
    os::install(i);
    place::install(i);
    reader::install(i);
    string::install(i);
}

