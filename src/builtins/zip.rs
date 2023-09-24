
//! An interface to zip files, courtesy of [zip].

use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use crate::base::*;
use crate::base::io::*;
use crate::interpreter::*;
use crate::builtins::util;

use zip;
use ouroboros::self_referencing;

#[derive(Clone, Default)]
struct ZipWriteOptions(zip::write::FileOptions);
value!(ZipWriteOptions: dyn query_interface::ObjectClone);

// TODO DateTime and last_modified_time

/// [zip::ZipArchive] wrapper.
#[derive(Clone)]
pub struct ZipArchive(Rc<RefCell<zip::ZipArchive<ReadSeeker>>>);
value!(ZipArchive: dyn query_interface::ObjectClone);

/// [zip::ZipWriter] wrapper.
#[derive(Clone)]
pub struct ZipWriter(Rc<RefCell<zip::ZipWriter<WriteSeeker>>>);
value!(ZipWriter: dyn query_interface::ObjectClone, dyn io::Write);

impl io::Write for ZipWriter {
   fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
       (*self.0).borrow_mut().write(buf)
   }

   fn flush(&mut self) -> io::Result<()> {
       (*self.0).borrow_mut().flush()
   }
}

/// [zip::ZipEntry] wrapper with embedded [ZipArchive] and no lifetime.
#[self_referencing]
pub struct ZipEntry {
    archive: ZipArchive,
    #[borrows(mut archive)]
    archive_borrow: std::cell::RefMut<'this, zip::ZipArchive<ReadSeeker>>,
    #[not_covariant]
    #[borrows(mut archive_borrow)]
    entry: zip::read::ZipFile<'this>,
}
value!(ZipEntry);

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
        let za = zip::ZipArchive::new(rs).map(|za| ZipArchive(Rc::new(RefCell::new(za)))).map_err(|e| e.to_string());
        i.stack_push_result(za);
        Ok(())
    });

    i.add_builtin("zip-archive-open-entry", |i: &mut Interpreter| {
        let path = i.stack_pop::<String>()?;
        let archive = i.stack_pop::<ZipArchive>()?.into_inner();
        let res =
            ZipEntry::try_new(archive,
                              |a| Ok((*a.0).borrow_mut()),
                              |b| b.by_name(path.as_ref()));
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
        i.stack_push(ZipWriter(Rc::new(RefCell::new(zip::ZipWriter::new(src)))));
        return Ok(())
    });

    i.add_builtin("zip-writer-start-file", |i: &mut Interpreter| {
        let opts = i.stack_pop::<ZipWriteOptions>()?.into_inner().0;
        let name = i.stack_pop::<String>()?.into_inner();
        let zw = i.stack_pop::<ZipWriter>()?.into_inner();
        let res = (*zw.0).borrow_mut().start_file(name, opts)
                    .map(|()| true).map_err(|e| e.to_string());
        i.stack_push(zw);
        i.stack_push_result(res);
        Ok(())
    });

    i.add_builtin("zip-writer-finish", |i: &mut Interpreter| {
        let zw = i.stack_pop::<ZipWriter>()?.into_inner();
        i.stack_push_result((*zw.0).borrow_mut().finish()
                            .map(WriteSeeker::into_inner).map_err(|e| e.to_string()));
        Ok(())
    });
}


