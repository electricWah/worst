
//! An interface to zip files, courtesy of [zip].

use std::io;
use crate::base::*;
use crate::base::io::*;
use crate::interpreter::*;
use crate::builtins::util;

use zip;
use ouroboros::self_referencing;

#[derive(Clone, Default)]
struct ZipWriteOptions(zip::write::FileOptions);
value!(ZipWriteOptions: {Clone});

// TODO DateTime and last_modified_time

/// [zip::ZipArchive] wrapper.
pub struct ZipArchive(zip::ZipArchive<ReadSeeker>);
value!(ZipArchive);

/// [zip::ZipWriter] wrapper.
pub struct ZipWriter(zip::ZipWriter<WriteSeeker>);
value!(ZipWriter: dyn io::Write);

impl io::Write for ZipWriter {
   fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
       self.0.write(buf)
   }

   fn flush(&mut self) -> io::Result<()> {
       self.0.flush()
   }
}

/// [zip::ZipEntry] wrapper with embedded [ZipArchive] and no lifetime.
#[self_referencing]
pub struct ZipEntry {
    archive: ZipArchive,
    #[not_covariant]
    #[borrows(mut archive)]
    entry: zip::read::ZipFile<'this>,
}
value!(ZipEntry: dyn io::Read);

impl io::Read for ZipEntry {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.with_entry_mut(|e| e.read(buf))
    }
}

/// Install embedded filesystem builtins.
pub fn install(i: &mut Interpreter) {
    util::add_const_type_builtin::<ZipArchive>(i, "<zip-archive>");
    util::add_const_type_builtin::<ZipEntry>(i, "<zip-entry>");
    util::add_const_type_builtin::<ZipWriter>(i, "<zip-writer>");
    util::add_const_type_builtin::<ZipWriteOptions>(i, "<zip-write-options>");
    i.add_builtin("zip-write-options", util::make_default::<ZipWriteOptions>);

    i.add_builtin("zip-archive-open", |i: &mut Interpreter| {
        let source = i.stack_pop_val()?;
        let Some(rs) = ReadSeeker::new(source) else {
            i.stack_push_error(false);
            return Ok(());
        };
        let za = zip::ZipArchive::new(rs).map(|za| ZipArchive(za)).map_err(|e| e.to_string());
        i.stack_push_result(za);
        Ok(())
    });

    i.add_builtin("zip-archive-open-entry", |i: &mut Interpreter| {
        let path = i.stack_pop::<String>()?;
        let archive = i.stack_pop::<ZipArchive>()?.into_inner();
        let res =
            ZipEntry::try_new(archive,
                              |a| a.0.by_name(path.as_ref()));
        i.stack_push_result(res.map_err(|e| e.to_string()));
        Ok(())
    });

    // TODO set compression method and level

    i.add_builtin("zip-write-options-set-unix-permissions", |i: &mut Interpreter| {
        let perms = i.stack_pop::<i64>()?.into_inner() as u32;
        let opts = i.stack_pop::<ZipWriteOptions>()?.into_inner();
        i.stack_push(ZipWriteOptions(opts.0.unix_permissions(perms)));
        Ok(())
    });

    i.add_builtin("zip-write-options-set-large-file", |i: &mut Interpreter| {
        let opts = i.stack_pop::<ZipWriteOptions>()?.into_inner();
        i.stack_push(ZipWriteOptions(opts.0.large_file(true)));
        Ok(())
    });

    i.add_builtin("zip-writer-new", |i: &mut Interpreter| {
        let source = i.stack_pop_val()?;
        let Some(src) = WriteSeeker::new(source) else {
            i.stack_push_error(false);
            return Ok(());
        };
        i.stack_push(ZipWriter(zip::ZipWriter::new(src)));
        return Ok(())
    });

    i.add_builtin("zip-writer-start-file", |i: &mut Interpreter| {
        let opts = i.stack_pop::<ZipWriteOptions>()?.into_inner().0;
        let name = i.stack_pop::<String>()?.into_inner();
        let mut zw = i.stack_pop::<ZipWriter>()?.into_inner();
        let res = zw.0.start_file(name, opts)
                    .map(|()| true).map_err(|e| e.to_string());
        i.stack_push(zw);
        i.stack_push_result(res);
        Ok(())
    });

    i.add_builtin("zip-writer-finish", |i: &mut Interpreter| {
        let mut zw = i.stack_pop::<ZipWriter>()?.into_inner();
        i.stack_push_result(zw.0.finish()
                            .map(WriteSeeker::into_inner).map_err(|e| e.to_string()));
        Ok(())
    });
}


