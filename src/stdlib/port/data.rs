
use super::port::IsPort;

use std::io;
use std::fmt;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use crate::data::*;
use crate::data::error::*;
use crate::interpreter::exec;

struct PortData {
    port: RefCell<Box<IsPort>>,
}

#[derive(Clone)]
pub struct Port(Rc<PortData>);

impl PartialEq for Port {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for Port {}

impl IsType for Port {
    fn get_type() -> Type {
        Type::new("port")
    }
}
impl HasType for Port {
    fn type_of(&self) -> Type {
        Self::port_type(&**self.0.port.borrow())
    }
}

impl ValueDescribe for Port {
    fn fmt_describe(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_show(fmt)
    }
}

impl DefaultValueClone for Port {}
impl DefaultValueEq for Port {}
impl ValueHash for Port {}
impl ValueShow for Port {}
impl Value for Port {}

impl Port {
    pub fn new<T: IsPort + 'static>(the: T) -> Self {
        let port = RefCell::new(Box::new(the));
        Port(Rc::new(PortData { port }))
    }

    pub fn stdin() -> Self {
        Port::new(io::stdin())
    }
    pub fn stdout() -> Self {
        Port::new(io::stdout())
    }
    pub fn stderr() -> Self {
        Port::new(io::stderr())
    }
}

impl Port {
    pub fn is_input(&self) -> bool {
        self.0.port.borrow().is_input()
    }
    pub fn is_output(&self) -> bool {
        self.0.port.borrow().is_output()
    }
    pub fn can_seek(&self) -> bool {
        self.0.port.borrow().can_seek()
    }

    fn port_type(port: &IsPort) -> Type {
        match port.port_type() {
            Some(ty) => ty,
            None => Type::new("port"),
        }
    }
}

impl Port {
    pub fn write(&mut self, data: Vec<u8>) -> exec::Result<()> {
        let r: exec::Result<RefMut<Box<IsPort>>> =
            self.0.port.try_borrow_mut()
            .or(Err(error::NotUnique().into()));
        let mut r = r?;
        let out: exec::Result<&mut io::Write> =
            r.as_output().ok_or(WrongPortType().into());
        out?.write_all(data.as_slice()).map_err(StdIoError::new)?;
        Ok(())
    }

    pub fn flush(&mut self) -> exec::Result<()> {
        let r: exec::Result<RefMut<Box<IsPort>>> =
            self.0.port.try_borrow_mut()
            .or(Err(error::NotUnique().into()));
        let mut r = r?;
        let out: exec::Result<&mut io::Write> =
            r.as_output().ok_or(WrongPortType().into());
        out?.flush().map_err(StdIoError::new)?;
        Ok(())
    }

    pub fn read_into(&mut self, buf: &mut Vec<u8>) -> exec::Result<usize> {
        let r: exec::Result<RefMut<Box<IsPort>>> =
            self.0.port.try_borrow_mut()
            .or(Err(error::NotUnique().into()));
        let mut r = r?;
        let inp: exec::Result<&mut io::Read> =
            r.as_input().ok_or(WrongPortType().into());
        Ok(inp?.read(buf).map_err(StdIoError::new)?)
    }

    pub fn read(&mut self, len: usize) -> exec::Result<Vec<u8>> {
        let r: exec::Result<RefMut<Box<IsPort>>> =
            self.0.port.try_borrow_mut()
            .or(Err(error::NotUnique().into()));
        let mut r = r?;
        let inp: exec::Result<&mut io::Read> =
            r.as_input().ok_or(WrongPortType().into());
        let mut v = vec![0; len];
        inp?.read_exact(&mut v).map_err(StdIoError::new)?;
        Ok(v)
    }

    pub fn seek(&mut self, seek: io::SeekFrom) -> exec::Result<u64> {
        let r: exec::Result<RefMut<Box<IsPort>>> =
            self.0.port.try_borrow_mut()
            .or(Err(error::NotUnique().into()));
        let mut r = r?;
        let inp: exec::Result<&mut io::Seek> =
            r.as_seekable().ok_or(WrongPortType().into());
        Ok(inp?.seek(seek).map_err(StdIoError::new)?)
    }

}

impl Port {
    pub fn is_unique(&self) -> bool {
        Rc::strong_count(&self.0) == 0
    }
}

#[derive(Debug)]
// TODO expected/actual
pub struct WrongPortType();
impl Error for WrongPortType {}

impl fmt::Display for WrongPortType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Wrong port type")
    }
}

