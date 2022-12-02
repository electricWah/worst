
#![warn(missing_docs)]

//! Hello and welcome to my programming language :)

pub mod base;
pub mod data;
pub mod interpreter;
pub mod reader;
pub mod builtins;

#[cfg(feature = "wasm")]
pub mod wasm;


