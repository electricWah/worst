
//! Filesystem bits, and a bundled filestore built in to the binary when enabled

use std::io;
use crate::impl_value;
use crate::base::*;
use crate::interpreter::{Interpreter, Handle};

#[cfg(feature = "enable_fs")]
pub(crate) mod fs {
    use super::*;
    use std::fs;
    use std::rc::Rc;
    use std::cell::RefCell;

    /// A reference-counted [fs::File] [Val].
    #[derive(Clone)]
    pub struct File {
        // path: String,
        handle: Rc<RefCell<fs::File>>,
    }

    /// Try to open the file.
    pub fn open_read(path: impl AsRef<std::path::Path>) -> io::Result<File> {
        Ok(File { handle: Rc::new(RefCell::new(fs::File::open(path)?)) })
    }

    impl_value!(File, value_read::<File>(), type_name("file"));
    impl io::Read for File {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.handle.as_ref().borrow_mut().read(buf)
        }
    }
}

#[cfg(feature = "bundled_fs_embed")]
mod embedded {
    use super::*;
    use include_dir::{include_dir, Dir};

    static EMBED_FS: Dir = include_dir!("$CARGO_MANIFEST_DIR/lib");

    #[derive(Clone)]
    pub struct File {
        // path: String,
        handle: &'static [u8],
    }

    pub fn open_read(path: impl AsRef<std::path::Path>) -> Option<File> {
        Some(File { handle: EMBED_FS.get_file(path)?.contents() })
    }

    impl_value!(File, value_read::<File>(), type_name("file"));
    impl io::Read for File {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.handle.read(buf)
        }
    }
}

/// Open a bundled file for reading using any enabled bundled fs feature
/// in order: zip (not yet implemented), embedded bundle
pub fn open_bundled_read(path: impl AsRef<std::path::Path>) -> Option<Val> {

    #[cfg(feature = "bundled_fs_embed")]
    if let Some(f) = embedded::open_read(path) {
        return Some(f.into());
    }

    None
}

/// Install `open-file/read` and `open-embedded-file/read` if enabled.
pub fn install(i: &mut Interpreter) {
    #[cfg(feature = "enable_fs")]
    i.define("open-file/read", |mut i: Handle| async move {
        let path = i.stack_pop::<String>().await;
        match fs::open_read(path.as_ref()) {
            Ok(f) => i.stack_push(f).await,
            Err(e) => {
                i.stack_push(format!("{}", e)).await;
                i.stack_push(false).await;
            },
        }
    });

    #[cfg(feature = "bundled_fs_embed")]
    i.define("open-embedded-file/read", |mut i: Handle| async move {
        let path = i.stack_pop::<String>().await;
        if let Some(f) = embedded::open_read(path.as_ref()) {
            i.stack_push(f).await;
        } else {
            i.stack_push(false).await;
        }
    });
}

