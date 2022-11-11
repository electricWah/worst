
//! Filesystem bits, and a bundled filestore built in to the binary when enabled

use std::io;
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use crate::base::*;
use crate::interpreter::{Interpreter, Handle};
use crate::builtins::util::*;

impl Value for fs::OpenOptions {}

async fn with_open_options(i: &mut Handle, f: impl FnOnce(&mut fs::OpenOptions, bool) -> &mut fs::OpenOptions) {
    let mut c = i.stack_pop::<fs::OpenOptions>().await.into_inner();
    f(&mut c, true);
    i.stack_push(c).await;
}


/// A reference-counted [fs::File] [Val].
#[derive(Clone)]
pub struct File {
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

impl io::Write for File {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.handle.as_ref().borrow_mut().write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.handle.as_ref().borrow_mut().flush()
    }
}

/// Install `open-file/read` and `open-embedded-file/read` if enabled.
pub fn install(i: &mut Interpreter) {

    i.define("file-open-options?", type_predicate::<fs::OpenOptions>);
    i.define("file-open-options", |mut i: Handle| async move {
        i.stack_push(fs::OpenOptions::new()).await;
    });
    i.define("file-open-options-set-append", |mut i: Handle| async move {
             with_open_options(&mut i, fs::OpenOptions::append).await;
    });
    i.define("file-open-options-set-create", |mut i: Handle| async move {
             with_open_options(&mut i, fs::OpenOptions::create).await;
    });
    i.define("file-open-options-set-create-new", |mut i: Handle| async move {
             with_open_options(&mut i, fs::OpenOptions::create_new).await;
    });
    i.define("file-open-options-set-read", |mut i: Handle| async move {
             with_open_options(&mut i, fs::OpenOptions::read).await;
    });
    i.define("file-open-options-set-truncate", |mut i: Handle| async move {
             with_open_options(&mut i, fs::OpenOptions::truncate).await;
    });
    i.define("file-open-options-set-write", |mut i: Handle| async move {
             with_open_options(&mut i, fs::OpenOptions::write).await;
    });

    i.define("file-open", |mut i: Handle| async move {
        let opts = i.stack_pop::<fs::OpenOptions>().await;
        let path = i.stack_pop::<String>().await;
        if let Some(handle) = or_io_error(&mut i, opts.open(path.as_ref())).await {
            i.stack_push(File { handle: Rc::new(RefCell::new(handle)) }).await;
        }
    });

    i.define("file-port?", type_predicate::<File>);
    i.define("file-port->string", port_to_string::<File>);
    i.define("file-port-read-range", port_read_range::<File>);
    i.define("file-port-write-range", port_write_range::<File>);
    i.define("file-port-flush", port_flush::<File>);
}

