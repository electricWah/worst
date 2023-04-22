
//! Basic stack-shuffling and control flow builtins

use crate::base::*;
use crate::interpreter::*;
use super::util;
use std::any::TypeId;

/// `quote` - Take the next thing in the definition body and put it on the stack.
pub fn quote(i: &mut Interpreter) -> BuiltinRet {
    let v = i.body_mut().pop();
    if let Some(v) = v {
        i.stack_push(v);
    } else {
        i.error("quote-nothing".to_symbol())?;
    }
    Ok(())
}

/// `drop` - Forget the value on top of the stack.
pub fn drop(i: &mut Interpreter) -> BuiltinRet {
    i.stack_pop_val()?;
    Ok(())
}

/// `clone` - Duplicate the value on top of the stack.
/// Due to reference counting this doesn't actually call [clone](Clone::clone)
/// on the inner value, though, so maybe it should be called `copy`.
pub fn clone(i: &mut Interpreter) -> BuiltinRet {
    let v = i.stack_pop_val()?;
    i.stack_push(v.clone());
    i.stack_push(v);
    Ok(())
}

/// `swap` - Swap the top two values on the stack.
pub fn swap(i: &mut Interpreter) -> BuiltinRet {
    let a = i.stack_pop_val()?;
    let b = i.stack_pop_val()?;
    i.stack_push(a);
    i.stack_push(b);
    Ok(())
}

/// `dig` - Rotate the top three values on the stack by taking the third value
/// and putting it on top.
pub fn dig(i: &mut Interpreter) -> BuiltinRet {
    let a = i.stack_pop_val()?;
    let b = i.stack_pop_val()?;
    let c = i.stack_pop_val()?;
    i.stack_push(b);
    i.stack_push(a);
    i.stack_push(c);
    Ok(())
}

/// `bury` - Rotate the top three values on the stack by taking the top value
/// and moving it down to third.
pub fn bury(i: &mut Interpreter) -> BuiltinRet {
    let a = i.stack_pop_val()?;
    let b = i.stack_pop_val()?;
    let c = i.stack_pop_val()?;
    i.stack_push(a);
    i.stack_push(c);
    i.stack_push(b);
    Ok(())
}

/// `not` - Replace a false value on the top of the stack with true,
/// and anything else with false.
pub fn not(i: &mut Interpreter) -> BuiltinRet {
    let v = i.stack_pop_val()?;
    let is = Some(&false) == v.downcast_ref::<bool>();
    i.stack_push(is);
    Ok(())
}

/// `error?` - Check if the value on top of the stack is an error with
/// [IsError::is_error], and put its result on top of the stack.
pub fn error_(i: &mut Interpreter) -> BuiltinRet {
    let eu = i.uniques_mut().get_type::<IsError>();
    let v = i.stack_top_val()?;
    i.stack_push(v.meta_ref().contains_val(&eu));
    Ok(())
}

fn eval_any_next(i: &mut Interpreter, v: Val) -> BuiltinRet {
    match v.try_downcast::<List>() {
        Ok(l) => i.eval_list_next(l),
        Err(v) => i.eval_next(v)?,
    }
    Ok(())
}

/// `eval` - Evaluate the value on top of the stack.
pub fn eval(i: &mut Interpreter) -> BuiltinRet {
    let e = i.stack_pop_val()?;
    eval_any_next(i, e)?;
    Ok(())
}

/// `eval-if` -
/// Evaluate the top value of the stack, but only if the next value is true;
/// otherwise drop both and do nothing.
pub fn eval_if(i: &mut Interpreter) -> BuiltinRet {
    let e = i.stack_pop_val()?;
    let cond = i.stack_pop::<bool>()?.into_inner();
    if cond {
        eval_any_next(i, e)?;
    }
    Ok(())
}

/// `eval-while` -
/// Evaluate the value on top of the stack.
/// Then, if the new value on top of the stack is true,
/// `eval-while` again with the original top value.
pub fn eval_while(i: &mut Interpreter) -> BuiltinRet {
    let e = i.stack_pop_val()?;
    let ne = e.clone();
    i.eval_next_once(move |i: &mut Interpreter| {
        if i.stack_pop::<bool>()?.into_inner() {
            i.stack_push(e);
            eval_while(i)?;
        }
        Ok(())
    });
    eval_any_next(i, ne)?;
    Ok(())
}

/// `uplevel` - Call the value on top of the stack as if in the parent stack frame.
pub fn uplevel(i: &mut Interpreter) -> BuiltinRet {
    i.enter_parent_frame()?;
    eval(i)?;
    Ok(())
}

/// `[ quote quote quote uplevel uplevel ] quote upquote definition-add`
pub fn upquote(i: &mut Interpreter) -> BuiltinRet {
    i.enter_parent_frame()?;
    quote(i)?;
    Ok(())
}

/// `value->constant` - Turn any value into a builtin that, when evaluated,
/// simply puts a copy of itself on the stack.
/// This lets you eval anything without having to handle lists and symbols specially.
pub fn value_to_constant(i: &mut Interpreter) -> BuiltinRet {
    let v = i.stack_pop_val()?;
    i.stack_push(Builtin::from(move |i: &mut Interpreter| {
        i.stack_push(v.clone());
        Ok(())
    }));
    Ok(())
}

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.add_builtin("quote", quote);
    i.add_builtin("clone", clone);
    i.add_builtin("drop", drop);
    i.add_builtin("dig", dig);
    i.add_builtin("bury", bury);
    i.add_builtin("eval", eval);
    i.add_builtin("eval-if", eval_if);
    i.add_builtin("eval-while", eval_while);
    i.add_builtin("uplevel", uplevel);
    i.add_builtin("upquote", upquote);
    i.add_builtin("value->constant", value_to_constant);
    i.add_builtin("swap", swap);
    i.add_builtin("not", not);
    i.add_builtin("error?", error_);
    i.add_builtin("pause", |i: &mut Interpreter| {
        let v = i.stack_pop_val()?;
        i.pause(v)?;
        Ok(())
    });
    i.add_builtin("stack-empty", |i: &mut Interpreter| {
        let v = i.stack_ref().is_empty();
        i.stack_push(v);
        Ok(())
    });
    i.add_builtin("stack-get", |i: &mut Interpreter| {
        let stack = i.stack_ref();
        i.stack_push(stack.clone());
        Ok(())
    });
    i.add_builtin("stack-set", |i: &mut Interpreter| {
        let stack = i.stack_pop::<List>()?;
        *i.stack_mut() = stack.into_inner();
        Ok(())
    });
    // i.add_builtin("call-stack", |i: &mut Interpreter| {
    //     let cs = i.call_stack_names()
            
    //         .into_iter().map(|x| {
    //             if let Some(x) = x { Val::from(x) }
    //             else { false.into() }
    //         }).collect::<Vec<Val>>();
    //     i.stack_push(List::from(cs));
    //     Ok(())
    // });
    i.add_builtin("code-next", |i: &mut Interpreter| {
        let next = i.body_mut().pop();
        i.stack_push_opterr(next);
        Ok(())
    });
    i.add_builtin("code-peek", |i: &mut Interpreter| {
        let next = i.body_ref().top().cloned();
        i.stack_push_opterr(next);
        Ok(())
    });
    // i.add_builtin("code-swap", |i: &mut Interpreter| {
    //     let mut swap = i.stack_pop::<List>()?;
    //     let body = i.body_mut();
    //     std::mem::swap(body, swap.as_mut());
    //     i.stack_push(swap);
    //     Ok(())
    // });

    util::add_type_predicate_builtin::<bool>(i, "bool?");
    i.add_builtin("bool-equal", util::equality::<bool>);
    i.add_builtin("bool-hash", util::value_hash::<bool>);
    util::add_type_predicate_builtin::<Symbol>(i, "symbol?");
    i.add_builtin("symbol-equal", util::equality::<Symbol>);
    i.add_builtin("symbol-hash", util::value_hash::<Symbol>);

    util::add_type_predicate_builtin::<Unique>(i, "unique?");
    i.add_builtin("unique-equal", util::equality::<Unique>);
    i.add_builtin("unique-hash", util::value_hash::<Unique>);
    i.add_builtin("make-unique", |i: &mut Interpreter| {
        let u = i.uniques_mut().create();
        i.stack_push(u);
        Ok(())
    });

    util::add_type_predicate_builtin::<Builtin>(i, "builtin?");

    util::add_type_predicate_builtin::<TypeId>(i, "type-id?");
    i.add_builtin("type-id-equal", util::equality::<TypeId>);
    i.add_builtin("type-id-hash", util::value_hash::<TypeId>);
    i.add_builtin("value-type-id", |i: &mut Interpreter| {
        let v = i.stack_pop_val()?;
        i.stack_push(v.val_type_id());
        Ok(())
    });
    i.add_builtin("type-id->unique", |i: &mut Interpreter| {
        let v = i.stack_pop::<TypeId>()?.into_inner();
        let u = i.uniques_mut().get_type_id(v);
        i.stack_push(u);
        Ok(())
    });
    i.add_builtin("unique-type-id?", |i: &mut Interpreter| {
        let is = i.stack_top::<Unique>()?.as_ref().is_type();
        i.stack_push(is);
        Ok(())
    });

    i.add_builtin("value-meta-entry", |i: &mut Interpreter| {
        let u = i.stack_pop::<Unique>()?;
        let v = i.stack_pop_val()?;
        i.stack_push_option(v.meta_ref().get_val(u.as_ref()));
        Ok(())
    });

    i.add_builtin("value-take-meta-entry", |i: &mut Interpreter| {
        let u = i.stack_pop::<Unique>()?;
        let mut v = i.stack_pop_val()?;
        let entry = v.meta_mut().take_val(u.as_ref());
        i.stack_push(v);
        i.stack_push_option(entry);
        Ok(())
    });

    i.add_builtin("value-insert-meta-entry", |i: &mut Interpreter| {
        let mv = i.stack_pop_val()?;
        let u = i.stack_pop::<Unique>()?.into_inner();
        let mut v = i.stack_pop_val()?;
        v.meta_mut().insert_val(u, mv);
        i.stack_push(v);
        Ok(())
    });

    i.add_builtin("value-set-error", |i: &mut Interpreter| {
        let v = i.stack_pop_val()?;
        i.stack_push_error(v);
        Ok(())
    });

    i.add_builtin("value-unset-error", |i: &mut Interpreter| {
        let mut v = i.stack_pop_val()?;
        let m = v.meta_mut();
        m.remove_val(&i.uniques_mut().get_type::<IsError>());
        i.stack_push(v);
        Ok(())
    });

    let enabled_features = List::from_iter(vec![
        #[cfg(feature = "enable_os")] "os".to_symbol(),
        #[cfg(feature = "enable_stdio")] "stdio".to_symbol(),
        #[cfg(feature = "enable_fs_os")] "fs-os".to_symbol(),
        #[cfg(feature = "enable_fs_embed")] "fs-embed".to_symbol(),
        #[cfg(feature = "enable_fs_zip")] "fs-zip".to_symbol(),
        #[cfg(feature = "enable_process")] "process".to_symbol(),
        #[cfg(feature = "wasm")] "wasm".to_symbol(),
    ]);
    i.add_builtin("features-enabled", move |i: &mut Interpreter| {
        i.stack_push(enabled_features.clone());
        Ok(())
    });

}

