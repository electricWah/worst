
use std::io;
use crate::data::*;

pub trait IsPort {
    fn is_input(&self) -> bool { false }
    fn is_output(&self) -> bool { false }
    fn as_input(&mut self) -> Option<&mut io::Read> { None }
    fn as_output(&mut self) -> Option<&mut io::Write> { None }
    fn can_seek(&self) -> bool { false }
    fn as_seekable(&mut self) -> Option<&mut io::Seek> { None }
    fn port_type(&self) -> Option<Type> { None }
}
downcast!(IsPort);

