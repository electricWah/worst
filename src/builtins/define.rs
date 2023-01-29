
//! `add_builtin` and other definition-related builtins

use crate::base::*;
use crate::interp2::*;
use crate::builtins::util;

// mod dispatch;
// mod dynamic;
// mod recursive;

struct NotDynamicResolvable;
impl Value for NotDynamicResolvable {}

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
        let res = i.resolve_definition(name);
        i.stack_push_option(res);
        Ok(())
    });

    i.add_builtin("value-set-name", |i: &mut Interpreter| {
        let name = i.stack_pop::<String>()?.into_inner();
        let mut v = i.stack_pop_val()?;
        v.meta_mut().push(DefineName::new(name));
        i.stack_push(v);
        Ok(())
    });

    // defset stuff

    i.add_builtin("definitions?", util::type_predicate::<DefSet>);
    i.add_builtin("definitions-empty", util::make_default::<DefSet>);

    i.add_builtin("definitions-local", |i: &mut Interpreter| {
        let defs = i.local_definitions();
        i.stack_push(defs.clone());
        Ok(())
    });
    i.add_builtin("definitions-all", |i: &mut Interpreter| {
        let defs = i.all_definitions();
        i.stack_push(defs);
        Ok(())
    });
    i.add_builtin("definitions->pairs", |i: &mut Interpreter| {
        let defs = i.stack_pop::<DefSet>()?;
        let pairs = defs.as_ref().iter().map(|(k, v)| (k.to_symbol(), v.clone()));
        i.stack_push(List::from_pairs(pairs));
        Ok(())
    });
    i.add_builtin("definitions-insert", |i: &mut Interpreter| {
        let def = i.stack_pop_val()?;
        let name = i.stack_pop::<Symbol>()?.into_inner();
        let mut defs = i.stack_pop::<DefSet>()?;
        defs.as_mut().insert(name.to_string(), def);
        i.stack_push(defs);
        Ok(())
    });
    i.add_builtin("definitions-append", |i: &mut Interpreter| {
        let b = i.stack_pop::<DefSet>()?;
        let mut a = i.stack_pop::<DefSet>()?;
        a.as_mut().append(b.as_ref());
        i.stack_push(a);
        Ok(())
    });
    i.add_builtin("definitions-append-locals", |i: &mut Interpreter| {
        let defs = i.stack_pop::<DefSet>()?;
        i.locals_mut().append(defs.as_ref());
        Ok(())
    });
    i.add_builtin("definitions-append-defenv", |i: &mut Interpreter| {
        let defs = i.stack_pop::<DefSet>()?;
        i.defenv_mut().append(defs.as_ref());
        Ok(())
    });

    i.add_builtin("value-has-definitions", |i: &mut Interpreter| {
        let v = i.stack_pop_val()?;
        i.stack_push(v.meta_ref().contains::<DefSet>());
        Ok(())
    });
    i.add_builtin("value-append-definitions", |i: &mut Interpreter| {
        let defs = i.stack_pop::<DefSet>()?.into_inner();
        let mut v = i.stack_pop_val()?;
        DefSet::upsert_meta(v.meta_mut(), |ds| ds.append(&defs));
        i.stack_push(v);
        Ok(())
    });

    // add a definition to a value's env
    i.add_builtin("value-definition-add", |i: &mut Interpreter| {
        let def = i.stack_pop_val()?;
        let name = i.stack_pop::<Symbol>()?.into_inner();
        let mut v = i.stack_pop_val()?;
        DefSet::upsert_meta(v.meta_mut(), |ds| ds.insert(name.to_string(), def));
        i.stack_push(v);
        Ok(())
    });

    i.add_builtin("value-set-not-dynamic-resolvable", |i: &mut Interpreter| {
        let mut v = i.stack_pop_val()?;
        v.meta_mut().push(NotDynamicResolvable);
        i.stack_push(v);
        Ok(())
    });

    // try resolving def, then recursively uplevel until found
    // dynamic-resolve would just look in locals I guess
    i.add_builtin("dynamic-resolve-any", |i: &mut Interpreter| {
        let name = i.stack_pop::<Symbol>()?;
        loop {
            // i.local_definitions().get(name.as_ref())
            if let Some(def) = i.resolve_definition(name.as_ref()) {
                if !def.meta_ref().contains::<NotDynamicResolvable>() {
                    i.stack_push(def);
                    break;
                }
            }
            if i.enter_parent_frame().is_err() {
                i.error(List::from(vec![
                    "dynamic-resolve-any".to_symbol().into(),
                    name.into(),
                ]))?;
                break;
            }
        }
        Ok(())
    });

    // try resolving def in locals, then recursively uplevel until found
    // or false + error
    i.add_builtin("dynamic-resolve-local", |i: &mut Interpreter| {
        let name = i.stack_pop::<Symbol>()?;
        loop {
            if let Some(def) = i.local_definitions().get(name.as_ref()) {
                if !def.meta_ref().contains::<NotDynamicResolvable>() {
                    i.stack_push(def.clone());
                    break;
                }
            }
            if i.enter_parent_frame().is_err() {
                i.stack_push(IsError::add(false));
                break;
            }
        }
        Ok(())
    });

    // dispatch::install(i);
    // dynamic::install(i);
    // recursive::install(i);
}

