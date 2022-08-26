
//! Input and output ports for reading and writing strings and bytes

use std::rc::Rc;
use std::io::{ self, Read, BufRead, Write };
use std::cell::RefCell;
// use std::collections::VecDeque;
use crate::impl_value;
use crate::base::*;
use crate::interpreter::{Interpreter, Handle};

struct OutputPort(RefCell<Box<dyn Write>>);
impl_value!(OutputPort);

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
impl_value!(BufReader, type_name("bufreader"));
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

/// Slurp entire port (i.e. until eof) into a string.
/// Pops a `T` and pushes the result,
/// either the string itself on success,
/// or an `error?` value on failure
/// (currently the string representation of the error).
pub async fn port_to_string<T: ImplValue + Read + Clone + 'static>(mut i: Handle) {
    let mut read = i.stack_pop::<T>().await;
    let mut s = String::new();
    match read.as_mut().read_to_string(&mut s) {
        Ok(_count) => {
            i.stack_push(s).await;
        },
        Err(e) => {
            i.stack_push(IsError::add(format!("{}", e))).await;
        },
    }
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

