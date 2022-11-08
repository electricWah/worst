
//! Input and output ports for reading and writing strings and bytes

use std::rc::Rc;
use std::io::{ self, Read, BufRead, Write };
use std::cell::RefCell;
// use std::collections::VecDeque;
use crate::base::*;
use crate::interpreter::{Interpreter, Handle};
use crate::builtins::bytevector::bytes_range_mut;

struct OutputPort(RefCell<Box<dyn Write>>);
impl Value for OutputPort {}

#[cfg(feature = "enable_stdio")]
impl OutputPort {
    fn stdout() -> Self {
        OutputPort(RefCell::new(Box::new(std::io::stdout())))
    }
    fn stderr() -> Self {
        OutputPort(RefCell::new(Box::new(std::io::stderr())))
    }
}

#[derive(Clone)]
struct BufReader(Rc<RefCell<dyn BufRead>>);
impl Value for BufReader {}
impl BufReader {
    #[cfg(feature = "enable_stdio")]
    fn stdin() -> Self {
        BufReader(Rc::new(RefCell::new(io::stdin().lock())))
    }
}
impl Read for BufReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.as_ref().borrow_mut().read(buf)
    }
}

/// Given an [io::Result], either return its `Ok` arm or put the error on the stack.
/// You should push one value to the stack in the `Some` return case.
pub async fn or_io_error<T>(i: &mut Handle, e: io::Result<T>) -> Option<T> {
    match e {
        Ok(v) => Some(v),
        Err(e) => {
            i.stack_push(IsError::add(format!("{}", e))).await;
            None
        }
    }
}

/// Slurp entire port (i.e. until eof) into a string.
/// Pops a `T` and pushes the result,
/// either the string itself on success,
/// or an `error?` value on failure
/// (currently the string representation of the error).
pub async fn port_to_string<T: Value + Read + Clone>(mut i: Handle) {
    let mut read = i.stack_pop::<T>().await;
    let mut s = String::new();
    if let Some(_count) = or_io_error(&mut i, read.as_mut().read_to_string(&mut s)).await {
        i.stack_push(s).await;
    }
}

/// Read a specified range from a port into a bytevector.
/// Creates a builtin with the following signature:
/// `port bytevector start len port-read-bytevector-range -> port bytevector read-count-or-error`
/// See [bytes_range_mut] for da rulez.
pub async fn port_read_range<T: Value + Read + Clone>(mut i: Handle) {
    let len = i.stack_pop::<i64>().await.into_inner();
    let start = i.stack_pop::<i64>().await.into_inner();
    let mut bytevector = i.stack_pop::<Vec<u8>>().await;
    let mut port = i.stack_pop::<T>().await;
    let mut bv = bytevector.as_mut();
    let mut range = bytes_range_mut(&mut bv, start, len);
    let res = 
        match port.as_mut().read(&mut range) {
            Ok(count) => Val::from(count as i64),
            Err(e) => IsError::add(format!("{}", e)),
        };
    i.stack_push(port).await;
    i.stack_push(bytevector).await;
    i.stack_push(res).await;
}

/// Install all these functions if enabled.
pub fn install(i: &mut Interpreter) {

    #[cfg(feature = "enable_stdio")] {
        i.define("current-output-port", |mut i: Handle| async move {
            i.stack_push(OutputPort::stdout()).await;
        });
        i.define("current-error-port", |mut i: Handle| async move {
            i.stack_push(OutputPort::stderr()).await;
        });

        // there is only one stdin, so Val it so it's Rc
        let stdin = Val::from(BufReader::stdin());
        i.define("current-input-port", move |mut i: Handle| {
            let stdin = stdin.clone();
            async move {
                i.stack_push(stdin.clone()).await;
            }
        });
    }

    i.define("port-write-string", |mut i: Handle| async move {
        let s = i.stack_pop::<String>().await.into_inner();
        let p = i.stack_top::<OutputPort>().await;
        write!(p.as_ref().0.borrow_mut(), "{}", s).unwrap();
    });

    i.define("port-flush", |i: Handle| async move {
        let p = i.stack_top::<OutputPort>().await;
        p.as_ref().0.borrow_mut().flush().unwrap();
    });

    i.define("buffered-port->string", port_to_string::<BufReader>);

    i.define("buffered-port-read-line", |mut i: Handle| async move {
        let p = i.stack_top::<BufReader>().await;
        let mut buf = String::new();
        p.as_ref().0.borrow_mut().read_line(&mut buf).unwrap();
        i.stack_push(buf).await;
    });
}

