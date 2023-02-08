
//! [DefEnv] and [DefSet] manipulation

use crate::base::*;
use crate::interp2::*;
use crate::builtins::util;

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.add_builtin("defset?", util::type_predicate::<DefSet>);
    i.add_builtin("defenv?", util::type_predicate::<DefEnv>);
    i.add_builtin("defset-empty", util::make_default::<DefSet>);
    i.add_builtin("defenv-empty", util::make_default::<DefEnv>);

    i.add_builtin("current-locals", |i: &mut Interpreter| {
        let defs = i.locals_ref().clone();
        i.stack_push(defs);
        Ok(())
    });
    i.add_builtin("current-defenv", |i: &mut Interpreter| {
        let defs = i.defenv_ref().clone();
        i.stack_push(defs);
        Ok(())
    });

    i.add_builtin("defset-merge", |i: &mut Interpreter| {
        let b = i.stack_pop::<DefSet>()?;
        let mut a = i.stack_pop::<DefSet>()?;
        a.as_mut().extend(b.as_ref());
        i.stack_push(a);
        Ok(())
    });
    i.add_builtin("defset-insert", |i: &mut Interpreter| {
        let def = i.stack_pop_val()?;
        let name = i.stack_pop::<Symbol>()?.into_inner();
        let mut defs = i.stack_pop::<DefSet>()?;
        defs.as_mut().insert(name.to_string(), def);
        i.stack_push(defs);
        Ok(())
    });
    i.add_builtin("defset-remove", |i: &mut Interpreter| {
        let name = i.stack_pop::<Symbol>()?.into_inner();
        let mut defs = i.stack_pop::<DefSet>()?;
        let old = defs.as_mut().remove(name.as_ref());
        i.stack_push(defs);
        i.stack_push_option(old);
        Ok(())
    });

    i.add_builtin("defenv-set-locals", |i: &mut Interpreter| {
        let set = i.stack_pop::<DefSet>()?.into_inner();
        let mut env = i.stack_pop::<DefEnv>()?;
        *env.as_mut().locals_mut() = set;
        i.stack_push(env);
        Ok(())
    });
    i.add_builtin("defenv-push-locals", |i: &mut Interpreter| {
        let set = i.stack_pop::<DefSet>()?.into_inner();
        let mut env = i.stack_pop::<DefEnv>()?;
        env.as_mut().push_locals(set);
        i.stack_push(env);
        Ok(())
    });
    i.add_builtin("defenv-pop-locals", |i: &mut Interpreter| {
        let mut env = i.stack_pop::<DefEnv>()?;
        let set = env.as_mut().pop_locals();
        i.stack_push_option(set);
        i.stack_push(env);
        Ok(())
    });

    i.add_builtin("current-defenv-push-locals", |i: &mut Interpreter| {
        let defs = i.stack_pop::<DefSet>()?.into_inner();
        i.defenv_mut().push_locals(defs);
        Ok(())
    });
    i.add_builtin("current-defenv-pop-locals", |i: &mut Interpreter| {
        let defs = i.defenv_mut().pop_locals();
        i.stack_push_option(defs);
        Ok(())
    });
    i.add_builtin("current-defenv-set", |i: &mut Interpreter| {
        let env = i.stack_pop::<DefEnv>()?.into_inner();
        *i.defenv_mut() = env;
        Ok(())
    });

    i.add_builtin("value-defenv", |i: &mut Interpreter| {
        let v = i.stack_pop_val()?;
        i.stack_push_option(v.meta_ref().first_val::<DefEnv>());
        Ok(())
    });
    i.add_builtin("value-set-defenv", |i: &mut Interpreter| {
        let defs = i.stack_pop::<DefEnv>()?.into_inner();
        let mut v = i.stack_pop_val()?;
        if v.meta_ref().contains::<DefEnv>() {
            dbg!("oh");
        }
        v.meta_mut().upsert_with(Default::default(), |d| *d = defs);
        i.stack_push(v);
        Ok(())
    });

    // TODO put this in worst
    i.add_builtin("value-definition-add", |i: &mut Interpreter| {
        let def = i.stack_pop_val()?;
        let name = i.stack_pop::<Symbol>()?.into_inner();
        let mut v = i.stack_pop_val()?;
        v.meta_mut().upsert_with(DefEnv::default(), |d| {
            d.locals_mut().insert(name.to_string(), def);
            d.push_locals(Default::default());
        });
        i.stack_push(v);
        Ok(())
    });

}


