
//! `add_builtin` and other definition-related builtins

use crate::base::*;
use crate::interpreter::*;

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
        let res = i.resolve_definition(name.as_ref());
        i.stack_push_option(res.cloned());
        Ok(())
    });

    // TODO use stack frame meta entry stuff and some non-special type

    i.add_builtin("value-set-not-dynamic-resolvable", |i: &mut Interpreter| {
        let mut v = i.stack_pop_val()?;
        v.meta_mut().insert_val(
            i.uniques_mut().get_type::<NotDynamicResolvable>(),
            NotDynamicResolvable.into());
        i.stack_push(v);
        Ok(())
    });

    // TODO instead redo recursive-dispatch so it doesn't depend on this
    // try resolving def, then recursively uplevel until found
    // dynamic-resolve would just look in locals I guess
    i.add_builtin("dynamic-resolve-any", |i: &mut Interpreter| {
        let tu = i.uniques_mut().get_type::<NotDynamicResolvable>();
        let name = i.stack_pop::<Symbol>()?;
        loop {
            // i.local_definitions().get(name.as_ref())
            if let Some(def) = i.resolve_definition(name.as_ref().as_ref()) {
                if !def.meta_ref().contains_val(&tu) {
                    i.stack_push(def.clone());
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
        let tu = i.uniques_mut().get_type::<NotDynamicResolvable>();
        let name = i.stack_pop::<Symbol>()?;
        loop {
            if let Some(def) = i.defenv_ref().get_local(name.as_ref().as_ref()) {
                if !def.meta_ref().contains_val(&tu) {
                    i.stack_push(def.clone());
                    break;
                }
            }
            if i.enter_parent_frame().is_err() {
                i.stack_push_error(false);
                break;
            }
        }
        Ok(())
    });
}

