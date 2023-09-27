
//! Core and auxiliary Worst functions implemented in Rust.

use crate::interpreter::*;

pub mod util;

pub mod bytevector;
pub mod core;
pub mod define;
pub mod defset;
pub mod fs;
pub mod i64map;
pub mod interpreter;
pub mod list;
pub mod numeric;
#[cfg(feature = "enable_os")]
pub mod os;
pub mod place;
pub mod port;
#[cfg(feature = "enable_process")]
pub mod process;
pub mod reader;
#[cfg(feature = "enable_stdio")]
pub mod stdio;
pub mod string;
#[cfg(feature = "enable_zip")]
pub mod zip;

/// Define all enabled builtins in the given [Interpreter].
pub fn install(i: &mut Interpreter) {
    bytevector::install(i);
    core::install(i);
    define::install(i);
    defset::install(i);
    fs::install(i);
    i64map::install(i);
    interpreter::install(i);
    list::install(i);
    numeric::install(i);
    #[cfg(feature = "enable_os")]
    os::install(i);
    place::install(i);
    port::install(i);
    #[cfg(feature = "enable_process")]
    process::install(i);
    reader::install(i);
    #[cfg(feature = "enable_stdio")]
    stdio::install(i);
    string::install(i);
    #[cfg(feature = "enable_zip")]
    zip::install(i);
}

