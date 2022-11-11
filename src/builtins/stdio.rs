
//! Input and output ports for reading and writing strings and bytes

use std::io;
use crate::base::*;
use crate::builtins::util::*;
use crate::interpreter::{Interpreter, Handle};

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

    i.define("stdin-port", |mut i: Handle| async move {
        i.stack_push(Stdin).await;
    });
    i.define("stdout-port", |mut i: Handle| async move {
        i.stack_push(Stdout).await;
    });
    i.define("stderr-port", |mut i: Handle| async move {
        i.stack_push(Stderr).await;
    });

    i.define("stdin-port-read-range", port_read_range::<Stdin>);

    i.define("stdout-port-write-string", port_write_string::<Stdout>);
    i.define("stdout-port-write-range", port_write_range::<Stdout>);
    i.define("stdout-port-flush", port_flush::<Stdout>);
    i.define("stderr-port-write-string", port_write_string::<Stderr>);
    i.define("stderr-port-write-range", port_write_range::<Stderr>);
    i.define("stderr-port-flush", port_flush::<Stderr>);

    i.define("stdin-port-read-line", |mut i: Handle| async move {
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_count) => i.stack_push(buf).await,
            Err(e) => i.stack_push(IsError::add(format!("{}", e))).await,
        }
    });
}

