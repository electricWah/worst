
//! Basic stack-shuffling and control flow builtins

use crate::base::*;
use crate::interp2::*;
use super::util;

/// `quote` - Take the next thing in the definition body and put it on the stack.
pub fn quote(i: &mut Interpreter) -> BuiltinRet {
    let v = i.body_next_val()?;
    i.stack_push(v);
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
    let v = i.stack_top_val()?;
    i.stack_push(IsError::is_error(&v));
    Ok(())
}

/// `eval` - Evaluate the value on top of the stack.
pub fn eval(i: &mut Interpreter) -> BuiltinRet {
    let e = i.stack_pop_val()?;
    match e.try_downcast::<List>() {
        Ok(l) => i.eval_list_next(l),
        Err(v) => i.eval_next(v)?,
    }
    Ok(())
}

/// `uplevel` - Call the value on top of the stack as if in the parent stack frame.
pub fn uplevel(i: &mut Interpreter) -> BuiltinRet {
    i.enter_parent_frame()?;
    eval(i)?;
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

/// `[ quote quote quote uplevel uplevel ] quote upquote definition-add`
pub fn upquote(i: &mut Interpreter) -> BuiltinRet {
    i.enter_parent_frame()?;
    let q = i.body_next_val()?;
    i.stack_push(q);
    Ok(())
}

/// `while` - Do things until don't
/// ```ignore
/// ; while [-> bool] [body ...]
/// add_builtin while [
///     upquote quote %%cond definition-add
///     upquote quote %%while-body definition-add
///     [
///         %%cond if [%%while-body %%loop] [[]] current-context-set-code
///     ] const %%loop
///     %%loop current-context-set-code
/// ]
/// ```
pub fn while_(i: &mut Interpreter) -> BuiltinRet {
    let cond = i.body_next::<List>()?;
    let body = i.body_next::<List>()?;
    i.stack_push(body);
    i.stack_push(cond);
    i.eval_next(Builtin::from(while_body))?;
    Ok(())
}

fn while_body(i: &mut Interpreter) -> BuiltinRet {
    let cond = i.stack_pop::<List>()?;
    let body = i.stack_pop::<List>()?;

    let cond2 = cond.clone();
    i.eval_next_once(move |i: &mut Interpreter| {
        if i.stack_pop::<bool>()?.into_inner() {
            let body2 = body.clone();
            i.eval_next_once(move |i: &mut Interpreter| {
                i.stack_push(body);
                i.stack_push(cond);
                i.eval_next(Builtin::from(while_body))?;
                Ok(())
            });
            i.eval_list_next(body2);
        }
        Ok(())
    });
    i.eval_list_next(cond2);
    Ok(())
}

/// `if` - Do or don't a thing and then don't or do another thing
/// ```ignore
/// ; bool if [if-true] [if-false]
/// add_builtin if [
///     upquote upquote
///     ; cond true false => false true cond
///     swap dig
///     quote swap when drop
///     quote eval uplevel
/// ]
/// ```
pub fn if_(i: &mut Interpreter) -> BuiltinRet {
    let ift = i.body_next::<List>()?;
    let iff = i.body_next::<List>()?;
    if i.stack_pop::<bool>()?.into_inner() {
        i.eval_list_next(ift);
    } else {
        i.eval_list_next(iff);
    }
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
    i.add_builtin("uplevel", uplevel);
    i.add_builtin("upquote", upquote);
    i.add_builtin("value->constant", value_to_constant);
    i.add_builtin("swap", swap);
    i.add_builtin("if", if_);
    i.add_builtin("while", while_);
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
    i.add_builtin("stack-get", |i: &mut Interpreter| {
        let s = i.stack_ref().clone();
        i.stack_push(s);
        Ok(())
    });

    i.add_builtin("bool?", util::type_predicate::<bool>);
    i.add_builtin("bool-equal", util::equality::<bool>);
    // i.add_builtin("bool-hash", util::value_hash::<bool>);
    i.add_builtin("symbol?", util::type_predicate::<Symbol>);
    i.add_builtin("symbol-equal", util::equality::<Symbol>);
    // i.add_builtin("symbol-hash", util::value_hash::<Symbol>);

    i.add_builtin("builtin?", util::type_predicate::<Builtin>);
    // i.add_builtin("builtin-name", |i: &mut Interpreter| {
    //     let b = i.stack_pop::<Builtin>();
    //     i.stack_push_option(Val::from(b).meta_ref().first_ref::<DefineMeta>()
    //                         .and_then(|m| m.name.as_ref()).map(|s| s.clone().to_symbol()));
    // });

    i.add_builtin("value-meta", |i: &mut Interpreter| {
        let v = i.stack_pop_val()?;
        i.stack_push(v.meta_ref().clone());
        Ok(())
    });

    i.add_builtin("value-set-error", |i: &mut Interpreter| {
        let v = i.stack_pop_val()?;
        i.stack_push(IsError::add(v));
        Ok(())
    });

    i.add_builtin("value-unset-error", |i: &mut Interpreter| {
        let mut v = i.stack_pop_val()?;
        let m = v.meta_mut();
        // reall errors
        while m.take_first::<IsError>().is_some() {}
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

