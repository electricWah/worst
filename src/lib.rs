
#![warn(missing_docs)]

//! Hello and welcome to my programming language :)

use std::sync::Once;

pub mod base;
pub mod interpreter;
pub mod reader;
pub mod builtins;

#[macro_use] extern crate query_interface;

static STD_TYPES: Once = Once::new();
/// Initialise one-time setup bits and stuff.
/// Register standard types as [Value] types.
pub fn init() {
    STD_TYPES.call_once(|| {
        use query_interface::*;
        use base::*;
        dynamic_interfaces! {
            String: dyn Value;
            bool: dyn Value;
            i64: dyn Value;
            f64: dyn Value;
            Vec<u8>: dyn Value;
        }
    })
}

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

