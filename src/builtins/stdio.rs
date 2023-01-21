
//! Input and output ports for reading and writing strings and bytes

use std::io;
use crate::base::*;
use crate::builtins::util::*;
use crate::interp2::*;

#[derive(Clone)]
struct Stdin;
impl Value for Stdin {}

#[derive(Clone)]
struct Stdout;
impl Value for Stdout {}

#[derive(Clone)]
struct Stderr;
impl Value for Stderr {}

impl io::Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        std::io::stdin().read(buf)
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        std::io::stdout().write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        std::io::stdout().flush()
    }
}

impl io::Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        std::io::stderr().write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        std::io::stderr().flush()
    }
}

/// Install all these functions if enabled.
pub fn install(i: &mut Interpreter) {

    i.add_builtin("stdin-port", |i: &mut Interpreter| {
        i.stack_push(Stdin);
        Ok(())
    });
    i.add_builtin("stdout-port", |i: &mut Interpreter| {
        i.stack_push(Stdout);
        Ok(())
    });
    i.add_builtin("stderr-port", |i: &mut Interpreter| {
        i.stack_push(Stderr);
        Ok(())
    });

    i.add_builtin("stdin-port-read-range", port_read_range::<Stdin>);

    i.add_builtin("stdout-port-write-string", port_write_string::<Stdout>);
    i.add_builtin("stdout-port-write-range", port_write_range::<Stdout>);
    i.add_builtin("stdout-port-flush", port_flush::<Stdout>);
    i.add_builtin("stderr-port-write-string", port_write_string::<Stderr>);
    i.add_builtin("stderr-port-write-range", port_write_range::<Stderr>);
    i.add_builtin("stderr-port-flush", port_flush::<Stderr>);

    i.add_builtin("stdin-port-read-line", |i: &mut Interpreter| {
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_count) => i.stack_push(buf),
            Err(e) => i.stack_push(IsError::add(format!("{}", e))),
        }
        Ok(())
    });
}

