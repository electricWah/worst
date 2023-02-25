
//! [DefSet] manipulation

use crate::base::*;
use crate::interpreter::*;
use crate::builtins::util;

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.add_builtin("defset?", util::type_predicate::<DefSet>);
    i.add_builtin("defset-empty", util::make_default::<DefSet>);

    i.add_builtin("current-locals-defset", |i: &mut Interpreter| {
        let defs = i.locals_ref();
        i.stack_push(defs.clone());
        Ok(())
    });
    i.add_builtin("current-ambient-defset", |i: &mut Interpreter| {
        let defs = i.ambients_ref();
        i.stack_push(defs.clone());
        Ok(())
    });
    i.add_builtin("current-defset", |i: &mut Interpreter| {
        let defs = i.static_defs();
        i.stack_push(defs);
        Ok(())
    });

    // i.add_builtin("defset->pairs", |i: &mut Interpreter| {
    //     let defs = i.stack_pop::<DefSet>()?;
    //     let pairs = defs.as_ref().iter().map(|(k, v)| (k.to_symbol(), v.clone()));
    //     i.stack_push(List::from_pairs(pairs));
    //     Ok(())
    // });
    i.add_builtin("defset-insert", |i: &mut Interpreter| {
        let def = i.stack_pop_val()?;
        let name = i.stack_pop::<Symbol>()?.into_inner();
        let mut defs = i.stack_pop::<DefSet>()?;
        defs.as_mut().insert(name, def);
        i.stack_push(defs);
        Ok(())
    });
    i.add_builtin("defset-remove", |i: &mut Interpreter| {
        let name = i.stack_pop::<Symbol>()?.into_inner();
        let mut defs = i.stack_pop::<DefSet>()?;
        let old = defs.as_mut().remove(name);
        i.stack_push(defs);
        i.stack_push_option(old);
        Ok(())
    });
    i.add_builtin("defset-append", |i: &mut Interpreter| {
        let b = i.stack_pop::<DefSet>()?;
        let mut a = i.stack_pop::<DefSet>()?;
        a.as_mut().append(b.as_ref());
        i.stack_push(a);
        Ok(())
    });

    i.add_builtin("current-locals-defset-set", |i: &mut Interpreter| {
        let defs = i.stack_pop::<DefSet>()?;
        *i.locals_mut() = defs.as_ref().clone();
        Ok(())
    });
    i.add_builtin("current-ambient-defset-set", |i: &mut Interpreter| {
        let defs = i.stack_pop::<DefSet>()?;
        *i.ambients_mut() = defs.as_ref().clone();
        Ok(())
    });

    i.add_builtin("value-defset", |i: &mut Interpreter| {
        let v = i.stack_pop_val()?;
        i.stack_push_option(v.meta_ref().first_val::<DefSet>());
        Ok(())
    });
    i.add_builtin("value-set-defset", |i: &mut Interpreter| {
        let defs = i.stack_pop::<DefSet>()?.into_inner();
        let mut v = i.stack_pop_val()?;
        DefSet::upsert_meta(v.meta_mut(), |ds| *ds = defs);
        i.stack_push(v);
        Ok(())
    });


}


