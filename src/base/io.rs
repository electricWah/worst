
//! A silly collection of combinations of [std::io] traits and implementations.
//! Use them in [value!] so that other things that expect
//! e.g. [std::io::Read] + [sts::io::Seek]
//! can successfully use them.

use std::io;
use super::value::*;

/// ReadSeek = Read + Seek.
/// Hack combo trait until I think of some better way to
/// implement multiple traits in a value.
pub trait ReadSeek: io::Read + io::Seek {}
impl<T: io::Read + io::Seek> ReadSeek for T {}

/// WriteSeek = Write + Seek.
pub trait WriteSeek: io::Write + io::Seek {}
impl<T: io::Write + io::Seek> WriteSeek for T {}

/// A struct wrapping a [Val] that implements [ReadSeek] and [Read] and [Seek].
pub struct ReadSeeker(Val);
/// A struct wrapping a [Val] that implements [WriteSeek] and [Write] and [Seek].
pub struct WriteSeeker(Val);

impl ReadSeeker {
    /// Wrap a [Val] if the underlying value implements [ReadSeek].
    pub fn new(mut v: Val) -> Option<ReadSeeker> {
        if v.as_trait_mut::<dyn ReadSeek>().is_some() {
            Some(ReadSeeker(v))
        } else {
            None
        }
    }
    /// Extract the inner [Val].
    pub fn into_inner(self) -> Val { self.0 }
}
impl io::Read for ReadSeeker {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let v = self.0.as_trait_mut::<dyn ReadSeek>().unwrap();
        v.read(buf)
    }
}
impl io::Seek for ReadSeeker {
    fn seek(&mut self, seek: io::SeekFrom) -> io::Result<u64> {
        let v = self.0.as_trait_mut::<dyn ReadSeek>().unwrap();
        v.seek(seek)
    }
}

impl WriteSeeker {
    /// Wrap a [Val] if the underlying value implements [WriteSeek].
    pub fn new(mut v: Val) -> Option<WriteSeeker> {
        if v.as_trait_mut::<dyn WriteSeek>().is_some() {
            Some(WriteSeeker(v))
        } else {
            None
        }
    }
    /// Extract the inner [Val].
    pub fn into_inner(self) -> Val { self.0 }
}
impl io::Write for WriteSeeker {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let v = self.0.as_trait_mut::<dyn WriteSeek>().unwrap();
        v.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        let v = self.0.as_trait_mut::<dyn WriteSeek>().unwrap();
        v.flush()
    }
}
impl io::Seek for WriteSeeker {
    fn seek(&mut self, seek: io::SeekFrom) -> io::Result<u64> {
        let v = self.0.as_trait_mut::<dyn WriteSeek>().unwrap();
        v.seek(seek)
    }
}


