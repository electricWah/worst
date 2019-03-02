
use std::rc::Rc;
use std::cell::RefCell;
use std::fs;
use std::io;
use crate::data::*;
use crate::interpreter::exec;
use crate::stdlib::port::{Port, IsPort};

#[derive(Debug, PartialEq, Eq, Clone, Default, Hash)]
pub struct OpenFileOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
}

impl StaticType for OpenFileOptions {
    fn static_type() -> Type {
        Type::new("open-file-options")
    }
}
impl ValueShow for OpenFileOptions {}
impl ValueDefaults for OpenFileOptions {}
impl Value for OpenFileOptions {}

impl OpenFileOptions {
    pub fn opts(&self) -> fs::OpenOptions {
        let mut opts = fs::OpenOptions::new();
        opts.read(self.read);
        opts.write(self.write);
        opts.append(self.append);
        opts.truncate(self.truncate);
        opts.create(self.create);
        opts.create_new(self.create_new);
        opts
    }

    pub fn new() -> Self {
        OpenFileOptions::default()
    }

    pub fn read(&mut self) {
        self.read = true;
    }
    pub fn write(&mut self) {
        self.write = true;
    }
    pub fn append(&mut self) {
        self.write = true;
        self.append = true;
    }
    pub fn truncate(&mut self) {
        self.truncate = true;
    }
    pub fn create(&mut self) {
        self.create = true;
    }
    pub fn create_new(&mut self) {
        self.create_new = true;
    }

    pub fn open(self, path: String) -> exec::Result<File> {
        File::open(self, path)
    }

}

#[derive(Debug, Clone)]
pub struct File {
    file: Rc<RefCell<fs::File>>,
    opts: OpenFileOptions,
}

impl StaticType for File {
    fn static_type() -> Type {
        Type::new("file")
    }
}
impl ValueEq for File {}
impl ValueShow for File {}
impl ValueHash for File {}
impl ValueDefaults for File {}
impl Value for File {}

impl File {
    fn open(opts: OpenFileOptions, path: String) -> exec::Result<Self> {
        let file = opts.opts().open(path).map_err(error::StdIoError::new)?;
        let file = Rc::new(RefCell::new(file));
        Ok(File { file, opts })
    }
    pub fn port(&self) -> Port {
        Port::new(self.clone())
    }
    pub fn sync_all(&mut self) -> exec::Result<()> {
        Ok(self.file.borrow_mut().sync_all().map_err(error::StdIoError::new)?)
    }
}

impl io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.borrow_mut().read(buf)
    }
}

impl io::Write for File {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.file.borrow_mut().write(data)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.file.borrow_mut().flush()
    }
}

impl io::Seek for File {
    fn seek(&mut self, from: io::SeekFrom) -> io::Result<u64> {
        self.file.borrow_mut().seek(from)
    }
}

impl IsPort for File {
    fn is_input(&self) -> bool {
        self.opts.read
    }
    fn is_output(&self) -> bool {
        self.opts.write
    }
    fn as_input(&mut self) -> Option<&mut io::Read> {
        Some(self)
    }
    fn as_output(&mut self) -> Option<&mut io::Write> {
        Some(self)
    }
    fn can_seek(&self) -> bool { true }
    fn as_seekable(&mut self) -> Option<&mut io::Seek> {
        Some(self)
    }
    fn port_type(&self) -> Option<Type> { Some(Type::new("file-port")) }
}

#[derive(Debug, Clone)]
pub struct FileMetadata(fs::Metadata);

impl StaticType for FileMetadata {
    fn static_type() -> Type {
        Type::new("file-info")
    }
}
impl ValueEq for FileMetadata {}
impl ValueHash for FileMetadata {}
impl ValueShow for FileMetadata {}
impl ValueDefaults for FileMetadata {}
impl Value for FileMetadata {}

impl FileMetadata {
    pub fn create(f: &File) -> io::Result<Self> {
        Ok(FileMetadata(f.file.borrow().metadata()?))
    }
    pub fn borrow(&self) -> &fs::Metadata {
        &self.0
    }
}

