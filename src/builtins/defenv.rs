
//! [DefEnv] manipulation

use crate::base::*;
use crate::interpreter::*;
use crate::builtins::util;

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    util::add_const_type_builtin::<DefEnv>(i, "<defenv>");
    i.add_builtin("defenv-empty", util::make_default::<DefEnv>);

    i.add_builtin("current-defenv", |i: &mut Interpreter| {
        let defs = i.defenv_ref().clone();
        i.stack_push(defs);
        Ok(())
    });
    i.add_builtin("current-defenv-set", |i: &mut Interpreter| {
        let env = i.stack_pop::<DefEnv>()?.into_inner();
        *i.defenv_mut() = env;
        Ok(())
    });

    i.add_builtin("defenv-insert-local", |i: &mut Interpreter| {
        let def = i.stack_pop_val()?;
        let name = i.stack_pop::<Symbol>()?.into_inner();
        let mut defs = i.stack_pop::<DefEnv>()?;
        defs.as_mut().insert_local(name.to_string(), def);
        i.stack_push(defs);
        Ok(())
    });
    i.add_builtin("defenv-lookup", |i: &mut Interpreter| {
        let name = i.stack_pop::<Symbol>()?.into_inner();
        let defs = i.stack_pop::<DefEnv>()?;
        i.stack_push_option(defs.as_ref().lookup(name.as_ref()).cloned());
        Ok(())
    });

    i.add_builtin("defenv-new-locals", |i: &mut Interpreter| {
        let mut defs = i.stack_pop::<DefEnv>()?;
        defs.as_mut().new_locals();
        i.stack_push(defs);
        Ok(())
    });

    // maybe use value-meta-entry with defenv type-id
    i.add_builtin("value-defenv", |i: &mut Interpreter| {
        let tu = i.uniques_mut().get_type::<DefEnv>();
        let v = i.stack_pop_val()?;
        i.stack_push_option(v.meta_ref().get_val(&tu));
        Ok(())
    });
    i.add_builtin("value-set-defenv", |i: &mut Interpreter| {
        let tu = i.uniques_mut().get_type::<DefEnv>();
        let defs = i.stack_pop::<DefEnv>()?;
        let mut v = i.stack_pop_val()?;
        v.meta_mut().insert_val(tu, defs.into());
        i.stack_push(v);
        Ok(())
    });

    // TODO put these in worst or something
    i.add_builtin("defenv-merge-locals", |i: &mut Interpreter| {
        let locs = i.stack_pop::<DefEnv>()?.into_inner();
        let mut env = i.stack_pop::<DefEnv>()?;
        env.as_mut().extend_locals(locs);
        i.stack_push(env);
        Ok(())
    });
    i.add_builtin("current-defenv-merge-locals", |i: &mut Interpreter| {
        let env = i.stack_pop::<DefEnv>()?.into_inner();
        i.defenv_mut().extend_locals(env);
        Ok(())
    });

    i.add_builtin("defenv-names-all", |i: &mut Interpreter| {
        let env = i.stack_pop::<DefEnv>()?.into_inner();
        let names = List::from_iter(env.iter().map(|(n, _, _)| Symbol::from(n)));
        i.stack_push(names);
        Ok(())
    });
}


