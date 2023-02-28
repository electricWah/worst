
//! Core and auxiliary Worst functions implemented in Rust.

use crate::interpreter::*;

pub mod util;

pub mod bytevector;
pub mod core;
pub mod defenv;
pub mod define;
pub mod doc;
pub mod fs;
pub mod i64map;
pub mod interpreter;
pub mod list;
pub mod numeric;
#[cfg(feature = "enable_os")]
pub mod os;
pub mod place;
#[cfg(feature = "enable_process")]
pub mod process;
pub mod reader;
#[cfg(feature = "enable_stdio")]
pub mod stdio;
pub mod string;
pub mod unique;

/// Define all enabled builtins in the given [Interpreter].
pub fn install(i: &mut Interpreter) {
    bytevector::install(i);
    core::install(i);
    defenv::install(i);
    define::install(i);
    doc::install(i);
    fs::install(i);
    i64map::install(i);
    interpreter::install(i);
    list::install(i);
    numeric::install(i);
    #[cfg(feature = "enable_os")]
    os::install(i);
    place::install(i);
    #[cfg(feature = "enable_process")]
    process::install(i);
    reader::install(i);
    #[cfg(feature = "enable_stdio")]
    stdio::install(i);
    string::install(i);
    unique::install(i);
}

