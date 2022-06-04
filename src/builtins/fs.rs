
use std::io;
use crate::impl_value;
use crate::base::*;
use crate::interpreter::{Interpreter, Handle};

#[cfg(feature = "builtin_fs")]
pub mod fs {
    use super::*;
    use std::fs;

    pub struct File {
        // path: String,
        handle: fs::File,
    }

    pub fn open_read(path: impl AsRef<std::path::Path>) -> io::Result<File> {
        Ok(File { handle: fs::File::open(path)? })
    }

    impl_value!(File, value_read::<File>(), type_name("file"));
    impl io::Read for File {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.handle.read(buf)
        }
    }
}

#[cfg(feature = "bundled_fs_embed")]
mod embedded {
    use super::*;
    use include_dir::{include_dir, Dir};

    static EMBED_FS: Dir = include_dir!("$WORST_BUNDLE_DIR"); // required for bundled_fs_embed feature

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

pub fn install(i: &mut Interpreter) {
    #[cfg(feature = "builtin_fs")]
    i.define("open-file/read", |mut i: Handle| async move {
        let path = i.stack_pop::<String>().await;
        match fs::open_read(path) {
            Ok(f) => i.stack_push(f).await,
            Err(e) => i.stack_push(format!("{}", e)).await,
        }
    });

    #[cfg(feature = "bundled_fs_embed")]
    i.define("open-embedded-file/read", |mut i: Handle| async move {
        let path = i.stack_pop::<String>().await;
        if let Some(f) = embedded::open_read(path) {
            i.stack_push(f).await;
        } else {
            i.stack_push(false).await;
        }
    });
}

