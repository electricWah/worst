
//! Basic stack-shuffling and control flow builtins

use crate::base::*;
use crate::interpreter::*;
use super::util;

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

/// `eval` - Evaluate the value on top of the stack.
pub fn eval(i: &mut Interpreter) -> BuiltinRet {
    let e = i.stack_pop_val()?;
    i.eval_any_next(e)?;
    Ok(())
}

/// `eval-if` -
/// Evaluate the top value of the stack, but only if the next value is true;
/// otherwise drop both and do nothing.
pub fn eval_if(i: &mut Interpreter) -> BuiltinRet {
    let e = i.stack_pop_val()?;
    let cond = i.stack_pop::<bool>()?.into_inner();
    if cond {
        i.eval_any_next(e)?;
    }
    Ok(())
}

/// `uplevel` - Evaluate the value on top of the stack as if in the parent stack frame.
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

/// `value-equal` - Test equality of the top two items on the stack.
pub fn value_equal(i: &mut Interpreter) -> BuiltinRet {
    let b = i.stack_pop_val()?;
    let a = i.stack_pop_val()?;
    let (Some(aeq), Some(beq)) =
        (a.as_trait_ref::<dyn query_interface::ObjectPartialEq>(),
         b.as_trait_ref::<dyn query_interface::Object>()) else {
        i.stack_push(false);
        return Ok(());
    };
    i.stack_push(aeq.obj_eq(beq));
    Ok(())
}

/// `value-compare` - Compare the top two items on the stack,
/// giving -1, 0, 1 or false (for incomparable).
pub fn value_compare(i: &mut Interpreter) -> BuiltinRet {
    let b = i.stack_pop_val()?;
    let a = i.stack_pop_val()?;
    let (Some(ac), Some(bc)) =
        (a.as_trait_ref::<dyn query_interface::ObjectPartialOrd>(),
         b.as_trait_ref::<dyn query_interface::Object>()) else {
        i.stack_push(false);
        return Ok(());
    };
    i.stack_push_option(ac.obj_partial_cmp(bc).map(|o| o as i64));
    Ok(())
}

/// `value->string-debug` - Turn the value into a string using [Debug].
/// Gives `#f` if it doesn't implement [Debug].
pub fn value_tostring_debug(i: &mut Interpreter) -> BuiltinRet {
    let v = i.stack_pop_val()?;
    let Some(vv) = v.as_trait_ref::<dyn std::fmt::Debug>() else {
        i.stack_push(false);
        return Ok(());
    };
    i.stack_push(format!("{:?}", vv));
    Ok(())
}

/// `value->string-display` - Turn the value into a string using [Display].
/// Gives `#f` if it doesn't implement [Display].
pub fn value_tostring_display(i: &mut Interpreter) -> BuiltinRet {
    let v = i.stack_pop_val()?;
    let Some(vv) = v.as_trait_ref::<dyn std::fmt::Display>() else {
        i.stack_push(false);
        return Ok(());
    };
    i.stack_push(format!("{}", vv));
    Ok(())
}


/// `value-hash` - Hash any built-in value type to [i64].
/// Pushes `no-hash` error instead if the type does not implement [query_interface::ObjectHash].
pub fn value_hash(i: &mut Interpreter) -> BuiltinRet {
    use std::hash::Hasher;
    use std::collections::hash_map::DefaultHasher;

    let v = i.stack_pop_val()?;
    let Some(h) = v.as_trait_ref::<dyn query_interface::ObjectHash>() else {
        i.stack_push_error("no-hash".to_symbol());
        return Ok(());
    };

    let mut hasher = DefaultHasher::new();
    h.obj_hash(&mut hasher);

    // just the bytes please
    let u = unsafe { std::mem::transmute::<u64, i64>(hasher.finish()) };
    i.stack_push(u);

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
    i.add_builtin("uplevel", uplevel);
    i.add_builtin("upquote", upquote);
    i.add_builtin("value->constant", value_to_constant);
    i.add_builtin("swap", swap);
    i.add_builtin("not", not);
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

    util::add_const_type_builtin::<IsError>(i, "<is-error>");
    util::add_const_type_builtin::<bool>(i, "<bool>");
    util::add_const_type_builtin::<Symbol>(i, "<symbol>");

    util::add_const_type_builtin::<Unique>(i, "<unique>");
    i.add_builtin("make-unique", |i: &mut Interpreter| {
        let u = i.uniques_mut().create();
        i.stack_push(u);
        Ok(())
    });

    util::add_const_type_builtin::<Builtin>(i, "<builtin>");

    util::add_const_type_builtin::<TypeId>(i, "<type-id>");
    i.add_builtin("value-type-id", |i: &mut Interpreter| {
        let v = i.stack_pop_val()?;
        i.stack_push(v.val_type_id());
        Ok(())
    });
    i.add_builtin("type-id->unique", |i: &mut Interpreter| {
        let v = i.stack_pop::<TypeId>()?.into_inner();
        let u = i.uniques_mut().get_type_id(v.0); // ew
        i.stack_push(u);
        Ok(())
    });
    i.add_builtin("unique-type-id?", |i: &mut Interpreter| {
        let is = i.stack_top::<Unique>()?.as_ref().is_type();
        i.stack_push(is);
        Ok(())
    });

    i.add_builtin("value-equal", value_equal);
    i.add_builtin("value-compare", value_compare);
    i.add_builtin("value-hash", value_hash);
    i.add_builtin("value->string-debug", value_tostring_debug);
    i.add_builtin("value->string-display", value_tostring_display);

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

    i.add_builtin("current-frame-meta-entry", |i: &mut Interpreter| {
        let u = i.stack_pop::<Unique>()?;
        let entry = i.frame_meta_ref().get_val(u.as_ref());
        i.stack_push_option(entry);
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

