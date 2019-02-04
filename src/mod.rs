
#[macro_use]
extern crate downcast;

// extern crate num_rational;
extern crate num_traits;

// #[macro_use]
// extern crate log;

// extern crate internship;

pub mod combo;

pub mod data;

pub mod parser_basic;
pub use parser_basic as parser;

pub mod interpreter;

pub mod stdlib;

