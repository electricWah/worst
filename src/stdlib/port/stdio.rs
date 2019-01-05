
use std::io;
use super::port::*;
use crate::data::*;

impl IsPort for io::Stdin {
    fn is_input(&self) -> bool { true }
    fn as_input(&mut self) -> Option<&mut io::Read> {
        Some(self)
    }
    fn port_type(&self) -> Option<Type> {
        Some(Type::new("stdin-port"))
    }
}

impl IsPort for io::Stdout {
    fn is_output(&self) -> bool { true }
    fn as_output(&mut self) -> Option<&mut io::Write> {
        Some(self)
    }
    fn port_type(&self) -> Option<Type> {
        Some(Type::new("stdout-port"))
    }
}

impl IsPort for io::Stderr {
    fn is_output(&self) -> bool { true }
    fn as_output(&mut self) -> Option<&mut io::Write> {
        Some(self)
    }
    fn port_type(&self) -> Option<Type> {
        Some(Type::new("stderr-port"))
    }
}


