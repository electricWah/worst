
//! [DefSet] manipulation

use crate::base::*;
use crate::interpreter::*;
use crate::builtins::util;

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    util::add_const_type_builtin::<DefSet>(i, "<defset>");
    i.add_builtin("defset-empty", util::make_default::<DefSet>);

    i.add_builtin("current-ambient-defset", |i: &mut Interpreter| {
        let defs = i.ambients_ref().clone();
        i.stack_push(defs);
        Ok(())
    });

    i.add_builtin("current-locals-defset", |i: &mut Interpreter| {
        let defs = i.locals_ref().clone();
        i.stack_push(defs);
        Ok(())
    });

    i.add_builtin("current-ambient-defset-set", |i: &mut Interpreter| {
        let defs = i.stack_pop::<DefSet>()?.into_inner();
        *i.ambients_mut() = defs;
        Ok(())
    });

    i.add_builtin("current-locals-defset-set", |i: &mut Interpreter| {
        let defs = i.stack_pop::<DefSet>()?.into_inner();
        *i.locals_mut() = defs;
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

    i.add_builtin("defset-get", |i: &mut Interpreter| {
        let name = i.stack_pop::<Symbol>()?.into_inner();
        let defs = i.stack_pop::<DefSet>()?;
        i.stack_push_option(defs.as_ref().get(name.as_ref()).cloned());
        Ok(())
    });

    i.add_builtin("defset-merge", |i: &mut Interpreter| {
        let b = i.stack_pop::<DefSet>()?;
        let mut a = i.stack_pop::<DefSet>()?;
        a.as_mut().merge(b.into_inner());
        i.stack_push(a);
        Ok(())
    });

    i.add_builtin("defset-names", |i: &mut Interpreter| {
        let defs = i.stack_pop::<DefSet>()?.into_inner();
        let names = List::from_iter(defs.iter().map(|(n, _)| Symbol::from(n)));
        i.stack_push(names);
        Ok(())
    });
}



