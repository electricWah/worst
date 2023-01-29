
//! Places are mutable things that can each store a value

use crate::interp2::*;
use crate::base::*;

/// Install `make-place`, `place-get` and `place-set` functions.
pub fn install(i: &mut Interpreter) {
    i.add_builtin("make-place", |i: &mut Interpreter| {
        let v = i.stack_pop_val()?;
        i.stack_push(Place::wrap(v));
        Ok(())
    });
    i.add_builtin("place-get", |i: &mut Interpreter| {
        let v = i.stack_pop::<Place>()?.as_ref().get();
        i.stack_push(v);
        Ok(())
    });
    i.add_builtin("place-set", |i: &mut Interpreter| {
        let v = i.stack_pop_val()?;
        let mut p = i.stack_pop::<Place>()?;
        p.as_mut().set(v);
        i.stack_push(p);
        Ok(())
    });
}

