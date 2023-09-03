
//! A read-only filesystem containing everything in the `lib/` directory,
//! courtesy of [include_dir].

use std::io;
use std::path::PathBuf;
use crate::base::*;
use crate::interpreter::*;
use crate::builtins::util;
use include_dir::{include_dir, Dir};

static EMBED_FS: Dir = include_dir!("$CARGO_MANIFEST_DIR/lib");

/// An open reference to a file found in the embedded filesystem.
#[derive(Clone)]
pub struct File {
    // path: String,
    handle: &'static [u8],
}
value!(File: dyn io::Read);

/// Open the path if it exists.
pub fn open_read(path: impl AsRef<std::path::Path>) -> Option<File> {
    Some(File { handle: EMBED_FS.get_file(path)?.contents() })
}

/// Open the path if it exists, and get its contents as a [&'static str].
pub fn open_read_str(path: impl AsRef<std::path::Path>) -> Option<&'static str> {
    EMBED_FS.get_file(path)?.contents_utf8()
}

impl io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.handle.read(buf)
    }
}

/// Install embedded filesystem builtins.
pub fn install(i: &mut Interpreter) {
    i.add_builtin("embedded-file-open", |i: &mut Interpreter| {
        let path = i.stack_pop::<PathBuf>()?;
        if let Some(f) = open_read(path.as_ref()) {
            i.stack_push(f);
        } else {
            i.stack_push_error(false);
        }
        Ok(())
    });
    util::add_const_type_builtin::<File>(i, "<embedded-file-port>");
}

