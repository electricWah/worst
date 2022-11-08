
//! Filesystem bits, and a bundled filestore built in to the binary when enabled

use std::io;
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use crate::base::*;
use crate::interpreter::{Interpreter, Handle};
use crate::builtins::util::*;
use crate::builtins::io::{ or_io_error, port_to_string, port_read_range };

/// A reference-counted [fs::File] [Val].
#[derive(Clone)]
pub struct File {
    // path: String,
    handle: Rc<RefCell<fs::File>>,
}
impl Value for File {}

/// Try to open the file.
pub fn open_read(path: impl AsRef<std::path::Path>) -> io::Result<File> {
    Ok(File { handle: Rc::new(RefCell::new(fs::File::open(path)?)) })
}

impl io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.handle.as_ref().borrow_mut().read(buf)
    }
}

/// Install `open-file/read` and `open-embedded-file/read` if enabled.
pub fn install(i: &mut Interpreter) {
    i.define("open-file/read", |mut i: Handle| async move {
        let path = i.stack_pop::<String>().await;
        if let Some(f) = or_io_error(&mut i, open_read(path.as_ref())).await {
            i.stack_push(f).await;
        }
    });

    i.define("file-port?", type_predicate::<File>);
    i.define("file-port->string", port_to_string::<File>);
    i.define("file-port-read-range", port_read_range::<File>);
}
