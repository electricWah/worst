
//! Reading code

use crate::base::*;
use crate::reader::*;
use crate::interpreter::*;
use crate::builtins::util;

/// Install a bunch of reader functions.
pub fn install(i: &mut Interpreter) {
    i.add_builtin("reader-empty", util::make_default::<Reader>);
    i.add_builtin("reader?", util::type_predicate::<Reader>);

    i.add_builtin("reader-complete", |i: &mut Interpreter| {
        let r = i.stack_pop::<Reader>()?.into_inner();
        match r.complete() {
            Ok(r) => i.stack_push_option(r),
            Err(e) => i.stack_push(IsError::add(format!("{:?}", e))),
        }
        Ok(())
    });
    i.add_builtin("reader-read-string", |i: &mut Interpreter| {
        let mut s = i.stack_pop::<String>()?;
        let mut r = i.stack_pop::<Reader>()?;
        let mut acc = vec![];
        let res = r.as_mut().read_into(&mut s.as_mut().chars(), &mut acc)
            .map(|()| true)
            .map_err(|e| format!("{:?}", e));
        i.stack_push(r);
        i.stack_push(List::from(acc));
        i.stack_push_result(res);
        Ok(())
    });

    i.add_builtin("read-string->list", |i: &mut Interpreter| {
        let mut s = i.stack_pop::<String>()?;
        i.stack_push_result(read_all(&mut s.as_mut().chars())
                            .map(List::from)
                            .map_err(|e| format!("{:?}", e)));
        Ok(())
    });
}

