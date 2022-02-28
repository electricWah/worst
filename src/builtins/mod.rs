
use crate::interpreter::Builder;

pub mod core;

pub fn install(i: Builder) -> Builder {
    core::install(i)
}


