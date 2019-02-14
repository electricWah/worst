
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;

use crate::stdlib::file::data::*;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<File>("file?");
    // interpreter.define_type_predicate::<>("file-port?");
    interpreter.define_type_predicate::<FileMetadata>("file-info?");

    interpreter.add_builtin("make-open-file-options", make_open_file_options);
    interpreter.add_builtin("open-file-read", open_file_read);
    interpreter.add_builtin("open-file-write", open_file_write);
    interpreter.add_builtin("open-file-append", open_file_append);
    interpreter.add_builtin("open-file-truncate", open_file_truncate);
    interpreter.add_builtin("open-file-create", open_file_create);
    interpreter.add_builtin("open-file-create-new", open_file_create_new);
    interpreter.add_builtin("open-file", open_file);
    interpreter.add_builtin("file-port", file_port);
    interpreter.add_builtin("file-sync", file_sync);

    // std::fs::Metadata
    interpreter.add_builtin("file-info", file_info);
    interpreter.add_builtin("file-info-directory?", is_file_info_directory);
    interpreter.add_builtin("file-info-file?", is_file_info_file);
    interpreter.add_builtin("file-info-symlink?", is_file_info_symlink);
    interpreter.add_builtin("file-info-length", file_info_length);
    interpreter.add_builtin("file-info-readonly?", is_file_info_readonly);
    // interpreter.add_builtin("file-info-modified-time", file_info_modified_time);
    // interpreter.add_builtin("file-info-accessed-time", file_info_accessed_time);
    // interpreter.add_builtin("file-info-created-time", file_info_created_time);

    // Filesystem stuff
    // interpreter.add_builtin("file-exists?", is_file_exists);
    // interpreter.add_builtin("delete-file", delete_file);
}

fn make_open_file_options(interpreter: &mut Interpreter) -> exec::Result<()> {
    interpreter.stack.push(Datum::new(OpenFileOptions::new()));
    Ok(())
}

fn open_file_read(interpreter: &mut Interpreter) -> exec::Result<()> {
    interpreter.stack.top_mut::<OpenFileOptions>()?.read();
    Ok(())
}

fn open_file_write(interpreter: &mut Interpreter) -> exec::Result<()> {
    interpreter.stack.top_mut::<OpenFileOptions>()?.write();
    Ok(())
}

fn open_file_append(interpreter: &mut Interpreter) -> exec::Result<()> {
    interpreter.stack.top_mut::<OpenFileOptions>()?.append();
    Ok(())
}

fn open_file_truncate(interpreter: &mut Interpreter) -> exec::Result<()> {
    interpreter.stack.top_mut::<OpenFileOptions>()?.truncate();
    Ok(())
}

fn open_file_create(interpreter: &mut Interpreter) -> exec::Result<()> {
    interpreter.stack.top_mut::<OpenFileOptions>()?.create();
    Ok(())
}

fn open_file_create_new(interpreter: &mut Interpreter) -> exec::Result<()> {
    interpreter.stack.top_mut::<OpenFileOptions>()?.create_new();
    Ok(())
}

fn open_file(interpreter: &mut Interpreter) -> exec::Result<()> {
    let path = interpreter.stack.pop::<String>()?;
    let opts = interpreter.stack.pop::<OpenFileOptions>()?;
    let f = opts.open(path)?;
    interpreter.stack.push(Datum::new(f));
    Ok(())
}

fn file_port(interpreter: &mut Interpreter) -> exec::Result<()> {
    let port = {
        let f = interpreter.stack.ref_at::<File>(0)?;
        f.port()
    };
    interpreter.stack.push(Datum::new(port));
    Ok(())
}

fn file_sync(interpreter: &mut Interpreter) -> exec::Result<()> {
    let f = interpreter.stack.top_mut::<File>()?;
    f.sync_all()?;
    Ok(())
}

fn file_info(interpreter: &mut Interpreter) -> exec::Result<()> {
    let info = FileMetadata::create(interpreter.stack.ref_at::<File>(0)?)?;
    interpreter.stack.push(Datum::new(info));
    Ok(())
}

fn is_file_info_directory(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = interpreter.stack.ref_at::<FileMetadata>(0)?.borrow().is_dir();
    interpreter.stack.push(Datum::new(r));
    Ok(())
}

fn is_file_info_file(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = interpreter.stack.ref_at::<FileMetadata>(0)?.borrow().is_file();
    interpreter.stack.push(Datum::new(r));
    Ok(())
}

fn is_file_info_symlink(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = interpreter.stack.ref_at::<FileMetadata>(0)?.borrow().file_type().is_symlink();
    interpreter.stack.push(Datum::new(r));
    Ok(())
}

fn file_info_length(interpreter: &mut Interpreter) -> exec::Result<()> {
    let len = interpreter.stack.ref_at::<FileMetadata>(0)?.borrow().len();
    interpreter.stack.push(Datum::new(isize::from_num(len)?));
    Ok(())
}

fn is_file_info_readonly(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = interpreter.stack.ref_at::<FileMetadata>(0)?.borrow().permissions().readonly();
    interpreter.stack.push(Datum::new(r));
    Ok(())
}

// fn file_info_modified_time(_interpreter: &mut Interpreter) -> exec::Result<()> {
//     Ok(())
// }

// fn file_info_accessed_time(_interpreter: &mut Interpreter) -> exec::Result<()> {
//     Ok(())
// }

// fn file_info_created_time(_interpreter: &mut Interpreter) -> exec::Result<()> {
//     Ok(())
// }

// fn is_file_exists(_interpreter: &mut Interpreter) -> exec::Result<()> {
//     Ok(())
// }

// fn delete_file(_interpreter: &mut Interpreter) -> exec::Result<()> {
//     Ok(())
// }

