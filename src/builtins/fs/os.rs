
//! Filesystem bits, and a bundled filestore built in to the binary when enabled

use std::io;
use std::fs;
use std::path::PathBuf;
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

/// Install filesystem functions: open options, etc.
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
        let path = i.stack_pop::<PathBuf>().await;
        if let Some(handle) = or_io_error(&mut i, opts.as_ref().open(path.as_ref())).await {
            i.stack_push(File { handle: Rc::new(RefCell::new(handle)) }).await;
        }
    });

    i.define("file-port?", type_predicate::<File>);
    i.define("file-port->string", port_to_string::<File>);
    i.define("file-port-read-range", port_read_range::<File>);
    i.define("file-port-write-range", port_write_range::<File>);
    i.define("file-port-flush", port_flush::<File>);

    i.define("fs-path-canonical", |mut i: Handle| async move {
        let p = i.stack_pop::<PathBuf>().await;
        if let Some(p) = or_io_error(&mut i, fs::canonicalize(p.as_ref())).await {
            i.stack_push(p).await;
        }
    });

    i.define("fs-copy", |mut i: Handle| async move {
        let dest = i.stack_pop::<PathBuf>().await;
        let src = i.stack_pop::<PathBuf>().await;
        if let Some(_len) = or_io_error(&mut i, fs::copy(src.as_ref(), dest.as_ref())).await {
            i.stack_push(dest).await;
        }
    });
    i.define("fs-move", |mut i: Handle| async move {
        let dest = i.stack_pop::<PathBuf>().await;
        let src = i.stack_pop::<PathBuf>().await;
        if let Some(_len) = or_io_error(&mut i, fs::rename(src.as_ref(), dest.as_ref())).await {
            i.stack_push(dest).await;
        }
    });

    i.define("fs-file-delete", |mut i: Handle| async move {
        let path = i.stack_pop::<PathBuf>().await;
        if let Some(_len) = or_io_error(&mut i, fs::remove_file(path.as_ref())).await {
            i.stack_push(true).await;
        }
    });
    i.define("fs-dir-delete-empty", |mut i: Handle| async move {
        let path = i.stack_pop::<PathBuf>().await;
        if let Some(_len) = or_io_error(&mut i, fs::remove_dir(path.as_ref())).await {
            i.stack_push(true).await;
        }
    });
    i.define("fs-dir-delete", |mut i: Handle| async move {
        let path = i.stack_pop::<PathBuf>().await;
        if let Some(_len) = or_io_error(&mut i, fs::remove_dir_all(path.as_ref())).await {
            i.stack_push(true).await;
        }
    });

    i.define("fs-dir-create", |mut i: Handle| async move {
        let name = i.stack_pop::<PathBuf>().await;
        if let Some(_len) = or_io_error(&mut i, fs::create_dir(name.as_ref())).await {
            i.stack_push(true).await;
        }
    });
    i.define("fs-dir-create-path", |mut i: Handle| async move {
        let name = i.stack_pop::<PathBuf>().await;
        if let Some(_len) = or_io_error(&mut i, fs::create_dir_all(name.as_ref())).await {
            i.stack_push(true).await;
        }
    });

    i.define("fs-dir-entries", |mut i: Handle| async move {
        let name = i.stack_pop::<PathBuf>().await;
        if let Some(rd) = or_io_error(&mut i, fs::read_dir(name.as_ref())).await {
            let mut l = vec![];
            for f in rd {
                if let Some(f) = or_io_error(&mut i, f).await {
                    l.push(Val::from(f.path()));
                } else {
                    return;
                }
            }
            i.stack_push(List::from(l)).await;
        }

    });

    // i.define("fs-metadata");
    // i.define("fs-link-target");
    // i.define("fs-link-metadata");

    // i.define("file-read-bytevector");
    // i.define("file-read-string");
    // // on unix enable numeric? or allow readonly bit only
    // i.define("fs-set-permissions");
    // i.define("file-write-bytevector");
    // i.define("file-write-string");

}

