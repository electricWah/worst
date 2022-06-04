
use std::io::{ self, Read, BufRead, Write };
use std::borrow::BorrowMut;
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

struct BufReader(RefCell<Box<dyn BufRead>>);
impl_value!(BufReader, value_read::<BufReader>(), type_name("bufreader"));
impl BufReader {
    #[cfg(feature = "enable_stdio")]
    fn stdin() -> Self {
        BufReader(RefCell::new(Box::new(io::stdin().lock())))
    }
}
impl Read for BufReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.borrow_mut().read(buf)
    }
}

struct StringReader(String);
impl_value!(StringReader, value_read::<StringReader>(), type_name("stringreader"));
impl Read for StringReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut head = self.0.split_off(buf.len());
        std::mem::swap(&mut head, &mut self.0);
        head.as_bytes().read(buf)
    }
}

pub fn install(i: &mut Interpreter) {

    #[cfg(feature = "enable_stdio")] {
        i.define("current-output-port", |mut i: Handle| async move {
            i.stack_push(OutputPort::stdout()).await;
        });
        i.define("current-error-port", |mut i: Handle| async move {
            i.stack_push(OutputPort::stderr()).await;
        });

        // there is only one stdin, so Val it so it's Rc
        i.stack_push(BufReader::stdin());
        let stdin = i.stack_pop_val().unwrap();

        i.define("current-input-port", move |mut i: Handle| {
            let stdin = stdin.clone();
            async move {
                i.stack_push(stdin.clone()).await;
            }
        });
    }

    i.define("port-write-string", |mut i: Handle| async move {
        let s = i.stack_pop::<String>().await;
        let pv = i.stack_pop_val().await;
        if let Some(p) = pv.downcast_ref::<OutputPort>() {
            let mut pp = p.0.borrow_mut();
            write!(pp, "{}", s).unwrap();
        } else {
            todo!()
        }
        i.stack_push(pv).await;
    });

    i.define("port-flush", |mut i: Handle| async move {
        let pv = i.stack_pop_val().await;
        if let Some(p) = pv.downcast_ref::<OutputPort>() {
            let mut pp = p.0.borrow_mut();
            pp.flush().unwrap();
        } else {
            todo!()
        }
        i.stack_push(pv).await;
    });

    i.define("can-read?", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        let can = ReadValue::can(&v);
        i.stack_push(v).await;
        i.stack_push(can).await;
    });

    i.define("port->string", |mut i: Handle| async move {
        let mut pv = i.stack_pop_val().await;
        let reader = ReadValue::try_read(&mut pv);
        if let Some(mut read) = reader {
            let mut s = String::new();
            match read.borrow_mut().read_to_string(&mut s) {
                Ok(_count) => {
                    i.stack_push(s).await;
                    i.stack_push(true).await;
                },
                Err(e) => {
                    i.stack_push(format!("{}", e)).await;
                    i.stack_push(false).await;
                },
            }
        } else {
            todo!("wrong type thinger")
        }
    });

    i.define("buffered-port-read-line", |mut i: Handle| async move {
        let pv = i.stack_pop_val().await;
        if let Some(p) = pv.downcast_ref::<BufReader>() {
            let mut buf = String::new();
            p.0.borrow_mut().read_line(&mut buf).unwrap();
            i.stack_push(pv).await;
            i.stack_push(buf).await;
        } else {
            todo!()
        }
    });

    i.define("open-string-input-port", |mut i: Handle| async move {
        let s = i.stack_pop::<String>().await;
        i.stack_push(StringReader(s)).await;
    });

}

