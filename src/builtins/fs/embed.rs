
//! A read-only filesystem containing everything in the `lib/` directory,
//! courtesy of [include_dir].

use std::io;
use crate::base::*;
use crate::interpreter::{Interpreter, Handle};
use crate::builtins::util::*;
use crate::builtins::io::port_to_string;
use include_dir::{include_dir, Dir};

static EMBED_FS: Dir = include_dir!("$CARGO_MANIFEST_DIR/lib");

/// An open reference to a file found in the embedded filesystem.
#[derive(Clone)]
pub struct File {
    // path: String,
    handle: &'static [u8],
}
impl Value for File {}

/// Open the path if it exists.
pub fn open_read(path: impl AsRef<std::path::Path>) -> Option<File> {
    Some(File { handle: EMBED_FS.get_file(path)?.contents() })
}

impl io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.handle.read(buf)
    }
}

/// Install embedded filesystem builtins.
pub fn install(i: &mut Interpreter) {
    i.define("open-embedded-file/read", |mut i: Handle| async move {
        let path = i.stack_pop::<String>().await;
        if let Some(f) = open_read(path.as_ref()) {
            i.stack_push(f).await;
        } else {
            i.stack_push(false).await;
        }
    });
    i.define("embedded-file-port?", type_predicate::<File>);
    i.define("embedded-file-port->string", port_to_string::<File>);
}


