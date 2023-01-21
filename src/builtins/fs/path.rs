
//! Filesystem bits: PathBuf conversion

use std::path::PathBuf;
use crate::base::*;
use crate::interp2::*;

impl Value for PathBuf {}

/// Install filesystem functions: path, open options, etc.
pub fn install(i: &mut Interpreter) {

    // i.add_builtin("fs-path?", type_predicate::<PathBuf>);
    // i.add_builtin("fs-path-equal", equality::<PathBuf>);
    i.add_builtin("string->fs-path", |i: &mut Interpreter| {
        let s = i.stack_pop::<String>()?;
        i.stack_push(PathBuf::from(s.as_ref()));
        Ok(())
    });
    i.add_builtin("try-fs-path->string", |i: &mut Interpreter| {
        let p = i.stack_pop::<PathBuf>()?;
        i.stack_push_option(p.as_ref().to_str().map(String::from));
        Ok(())
    });
    i.add_builtin("fs-path->string-lossy", |i: &mut Interpreter| {
        let p = i.stack_pop::<PathBuf>()?;
        i.stack_push(String::from(p.as_ref().to_string_lossy()));
        Ok(())
    });

    i.add_builtin("fs-path-absolute", |i: &mut Interpreter| {
        let p = i.stack_pop::<PathBuf>()?;
        i.stack_push(p.as_ref().is_absolute());
        Ok(())
    });
    i.add_builtin("fs-path-parent", |i: &mut Interpreter| {
        let p = i.stack_pop::<PathBuf>()?;
        i.stack_push_option(p.as_ref().parent().map(PathBuf::from));
        Ok(())
    });
    i.add_builtin("fs-path-concat", |i: &mut Interpreter| {
        let p = i.stack_pop::<PathBuf>()?;
        let mut base = i.stack_pop::<PathBuf>()?;
        base.as_mut().push(p.as_ref());
        i.stack_push(base);
        Ok(())
    });
}

