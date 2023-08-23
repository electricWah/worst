
#![warn(missing_docs)]

//! Hello and welcome to my programming language :)

pub mod base;
pub mod interpreter;
pub mod reader;
pub mod builtins;

#[macro_use] extern crate query_interface;

#[cfg(feature = "wasm")]
pub mod wasm;


#[cfg(feature = "enable_fs_embed")]
/// Create an interpreter that runs `worst/prelude.w` from the embedded filesystem.
/// Panics if it is missing or malformed.
pub fn embedded() -> interpreter::Interpreter {
    let file = builtins::fs::embed::open_read_str("base/prelude.w")
        .expect("embedded worst/prelude.w");
    let prelude = reader::read_all(&mut file.chars()).unwrap();
    interpreter::Interpreter::new(prelude)
}

