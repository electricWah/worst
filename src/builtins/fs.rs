
//! Access to filesystem(s).

use crate::interpreter::Interpreter;

#[cfg(feature = "enable_fs_os")]
pub mod os;
#[cfg(feature = "enable_fs_embed")]
pub mod embed;

/// Install all enabled filesystem modules.
pub fn install(i: &mut Interpreter) {
    #[cfg(feature = "enable_fs_os")]
    os::install(i);
    #[cfg(feature = "enable_fs_embed")]
    embed::install(i);
}

