
//! Reading code

use crate::base::*;
use crate::reader::*;
use crate::interp2::*;

/// Install a bunch of reader functions.
pub fn install(i: &mut Interpreter) {
    i.add_builtin("reader-empty", |i: &mut Interpreter| {
        i.stack_push(Reader::new());
        Ok(())
    });
    i.add_builtin("reader-set-eof", |i: &mut Interpreter| {
        let mut r = i.stack_pop::<Reader>()?;
        r.as_mut().set_eof();
        i.stack_push(r);
        Ok(())
    });
    i.add_builtin("reader-write-string", |i: &mut Interpreter| {
        let mut s = i.stack_pop::<String>()?;
        let mut r = i.stack_pop::<Reader>()?;
        r.as_mut().write(&mut s.as_mut().chars());
        i.stack_push(r);
        Ok(())
    });
    // -> val #t | err #f | #f #f (eof)
    i.add_builtin("reader-next", |i: &mut Interpreter| {
        let mut r = i.stack_pop::<Reader>()?;
        let res = r.as_mut().read_next();
        i.stack_push(r);
        match res {
            Ok(Some(v)) => {
                i.stack_push(v);
                i.stack_push(true);
            },
            Ok(None) => {
                i.stack_push(false);
                i.stack_push(false);
            },
            Err(e) => {
                i.stack_push(e);
                i.stack_push(false);
            },
        }
        Ok(())
    });

    i.add_builtin("read-string->list", |i: &mut Interpreter| {
        let mut s = i.stack_pop::<String>()?;
        match read_all(&mut s.as_mut().chars()) {
            Ok(v) => i.stack_push(List::from(v)),
            Err(e) => i.stack_push(IsError::add(format!("{:?}", e))),
        }
        Ok(())
    });
}

