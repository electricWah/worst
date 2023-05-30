
//! `add_builtin` and other definition-related builtins

use crate::base::*;
use crate::interpreter::*;

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.add_builtin("definition-add", |i: &mut Interpreter| {
        let name = i.stack_pop::<Symbol>()?.into_inner();
        let def = i.stack_pop_val()?;
        i.add_definition(name, def);
        Ok(())
    });
    i.add_builtin("definition-resolve", |i: &mut Interpreter| {
        let name = i.stack_pop::<Symbol>()?.into_inner();
        let res = i.resolve_definition(name.as_ref());
        i.stack_push_option(res.cloned());
        Ok(())
    });
}

