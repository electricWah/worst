
use crate::data::*;
use crate::parser::*;
use crate::interpreter::Interpreter;
use crate::interpreter::command::*;
use crate::interpreter::exec;
use crate::stdlib::enumcommand::*;

use crate::stdlib::file::data::*;

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FileOp {
    // Filehandle stuff
    // std::fs::OpenOptions
    MakeOpenFileOptions,
    OpenFileRead,
    OpenFileWrite,
    OpenFileAppend,
    OpenFileTruncate,
    OpenFileCreate,
    OpenFileCreateNew,
    OpenFile,
    FilePort,
    FileSync, // Shouldn't need this
    IsFile,
    IsFilePort,

    // std::fs::Metadata
    FileInfo,
    IsFileInfoDirectory,
    IsFileInfoFile,
    IsFileInfoSymlink,
    FileInfoLength,
    IsFileInfoReadonly,
    FileInfoModifiedTime,
    FileInfoAccessedTime,
    FileInfoCreatedTime,
    IsFileInfo,

    // Filesystem stuff
    FileExists,
    DeleteFile,
}

impl EnumCommand for FileOp {
    fn as_str(&self) -> &str {
        use self::FileOp::*;
        match self {
            MakeOpenFileOptions => "make-open-file-options",
            OpenFileRead => "open-file-read",
            OpenFileWrite => "open-file-write",
            OpenFileAppend => "open-file-append",
            OpenFileTruncate => "open-file-truncate",
            OpenFileCreate => "open-file-create",
            OpenFileCreateNew => "open-file-create-new",
            OpenFile => "open-file",
            FilePort => "file-port",
            FileSync => "file-sync",
            IsFile => "file?",
            IsFilePort => "file-port?",

            // std::fs::Metadata
            FileInfo => "file-info",
            IsFileInfoDirectory => "file-info-directory?",
            IsFileInfoFile => "file-info-file?",
            IsFileInfoSymlink => "file-info-symlink?",
            FileInfoLength => "file-info-length",
            IsFileInfoReadonly => "file-info-readonly?",
            FileInfoModifiedTime => "file-info-modified-time",
            FileInfoAccessedTime => "file-info-accessed-time",
            FileInfoCreatedTime => "file-info-created-time",
            IsFileInfo => "file-info?",

            // Filesystem stuff
            FileExists => "file-exists?",
            DeleteFile => "delete-file",
        }
    }
    fn last() -> Self { FileOp::DeleteFile }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

impl Command for FileOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::FileOp::*;
        match self {
            MakeOpenFileOptions => {
                interpreter.stack.push(Datum::build().with_source(source).ok(OpenFileOptions::new()));
            },
            OpenFileRead => {
                interpreter.stack.top_mut::<OpenFileOptions>()?.read();
            },
            OpenFileWrite => {
                interpreter.stack.top_mut::<OpenFileOptions>()?.write();
            },
            OpenFileAppend => {
                interpreter.stack.top_mut::<OpenFileOptions>()?.append();
            },
            OpenFileTruncate => {
                interpreter.stack.top_mut::<OpenFileOptions>()?.truncate();
            },
            OpenFileCreate => {
                interpreter.stack.top_mut::<OpenFileOptions>()?.create();
            },
            OpenFileCreateNew => {
                interpreter.stack.top_mut::<OpenFileOptions>()?.create_new();
            },
            OpenFile => {
                let path = interpreter.stack.pop::<String>()?;
                let opts = interpreter.stack.pop::<OpenFileOptions>()?;
                let f = opts.open(path)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(f));
            },
            IsFile => {
                let r = interpreter.stack.type_predicate::<File>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            FilePort => {
                let port = {
                    let f = interpreter.stack.ref_at::<File>(0)?;
                    f.port()
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(port));
            },
            FileSync => {
                let f = interpreter.stack.top_mut::<File>()?;
                f.sync_all()?;
            },
            FileInfo => {
                let info = FileMetadata::create(interpreter.stack.ref_at::<File>(0)?)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(info));
            },
            FileInfoLength => {
                let len = interpreter.stack.ref_at::<FileMetadata>(0)?.borrow().len();
                interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(len)));
            },
            IsFileInfoReadonly => {
                let r = interpreter.stack.ref_at::<FileMetadata>(0)?.borrow().permissions().readonly();
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            IsFileInfoFile => {
                let r = interpreter.stack.ref_at::<FileMetadata>(0)?.borrow().is_file();
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            IsFileInfoDirectory => {
                let r = interpreter.stack.ref_at::<FileMetadata>(0)?.borrow().is_dir();
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            IsFileInfoSymlink => {
                let r = interpreter.stack.ref_at::<FileMetadata>(0)?.borrow().file_type().is_symlink();
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            IsFileInfo => {
                let r = interpreter.stack.type_predicate::<FileMetadata>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            _ => return Err(error::NotImplemented().into()),
        }
        Ok(())
    }
}

