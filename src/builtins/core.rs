
//! Basic stack-shuffling and control flow builtins

use crate::base::*;
use crate::interpreter::{Interpreter, Handle, Builtin, DefineMeta};
use super::util;

/// `quote` - Take the next thing in the definition body and put it on the stack.
pub async fn quote(mut i: Handle) {
    let q = i.quote_val().await;
    i.stack_push(q).await;
}

/// `drop` - Forget the value on top of the stack.
pub async fn drop(mut i: Handle) {
    i.stack_pop_val().await;
}

/// `clone` - Duplicate the value on top of the stack.
/// Due to reference counting this doesn't actually call [clone](Clone::clone)
/// on the inner value, though, so maybe it should be called `copy`.
pub async fn clone(mut i: Handle) {
    let v = i.stack_pop_val().await;
    i.stack_push(v.clone()).await;
    i.stack_push(v).await;
}

/// `swap` - Swap the top two values on the stack.
pub async fn swap(mut i: Handle) {
    let a = i.stack_pop_val().await;
    let b = i.stack_pop_val().await;
    i.stack_push(a).await;
    i.stack_push(b).await;
}

/// `dig` - Rotate the top three values on the stack by taking the third value
/// and putting it on top.
pub async fn dig(mut i: Handle) {
    let a = i.stack_pop_val().await;
    let b = i.stack_pop_val().await;
    let c = i.stack_pop_val().await;
    i.stack_push(b).await;
    i.stack_push(a).await;
    i.stack_push(c).await;
}

/// `bury` - Rotate the top three values on the stack by taking the top value
/// and moving it down to third.
pub async fn bury(mut i: Handle) {
    let a = i.stack_pop_val().await;
    let b = i.stack_pop_val().await;
    let c = i.stack_pop_val().await;
    i.stack_push(a).await;
    i.stack_push(c).await;
    i.stack_push(b).await;
}

/// `not` - Replace a false value on the top of the stack with true,
/// and anything else with false.
pub async fn not(mut i: Handle) {
    let v = i.stack_pop_val().await;
    let is = Some(&false) == v.downcast_ref::<bool>();
    i.stack_push(is).await;
}

/// `error?` - Check if the value on top of the stack is an error with
/// [IsError::is_error], and put its result on top of the stack.
pub async fn error_(mut i: Handle) {
    let v = i.stack_top_val().await;
    i.stack_push(IsError::is_error(&v)).await;
}

/// `eval` - Evaluate the value on top of the stack. See [Handle::eval].
pub async fn eval(mut i: Handle) {
    let e = i.stack_pop_val().await;
    i.eval(e).await;
}

/// `uplevel` - Call the value on top of the stack as if in the parent stack frame.
pub async fn uplevel(mut i: Handle) {
    i.uplevel(|mut i: Handle| async move {
        let c = i.stack_pop_val().await;
        i.eval(c).await;
    }).await;
}

/// `value->constant` - Turn any value into a builtin that, when evaluated,
/// simply puts a copy of itself on the stack.
/// This lets you eval anything without having to handle lists and symbols specially.
pub async fn value_to_constant(mut i: Handle) {
    let v = i.stack_pop_val().await;
    i.stack_push(Builtin::from(move |mut i: Handle| {
        let vv = v.clone();
        async move {
            i.stack_push(vv.clone()).await;
        }
    })).await;
}

/// `[ quote quote quote uplevel uplevel ] quote upquote definition-add`
pub async fn upquote(mut i: Handle) {
    i.uplevel(|mut i: Handle| async move {
        let q = i.quote_val().await;
        i.stack_push(q).await;
    }).await;
}

/// `while` - Do things until don't
/// ```ignore
/// ; while [-> bool] [body ...]
/// define while [
///     upquote quote %%cond definition-add
///     upquote quote %%while-body definition-add
///     [
///         %%cond if [%%while-body %%loop] [[]] current-context-set-code
///     ] const %%loop
///     %%loop current-context-set-code
/// ]
/// ```
pub async fn while_(mut i: Handle) {
    let cond = i.quote_val().await;
    let body = i.quote_val().await;
    loop {
        i.eval(cond.clone()).await;
        if !i.stack_pop::<bool>().await.into_inner() { break; }
        i.eval(body.clone()).await;
    }
}

/// `if` - Do or don't a thing and then don't or do another thing
/// ```ignore
/// ; bool if [if-true] [if-false]
/// define if [
///     upquote upquote
///     ; cond true false => false true cond
///     swap dig
///     quote swap when drop
///     quote eval uplevel
/// ]
/// ```
pub async fn if_(mut i: Handle) {
    let ift = i.quote_val().await;
    let iff = i.quote_val().await;
    if i.stack_pop::<bool>().await.into_inner() {
        i.eval(ift).await;
    } else {
        i.eval(iff).await;
    }
}

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.define("quote", quote);
    i.define("clone", clone);
    i.define("drop", drop);
    i.define("dig", dig);
    i.define("bury", bury);
    i.define("eval", eval);
    i.define("uplevel", uplevel);
    i.define("upquote", upquote);
    i.define("value->constant", value_to_constant);
    i.define("swap", swap);
    i.define("if", if_);
    i.define("while", while_);
    i.define("not", not);
    i.define("error?", error_);
    i.define("pause", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        i.pause(v).await;
    });
    i.define("stack-empty", |mut i: Handle| async move {
        let v = i.stack_empty().await;
        i.stack_push(v).await;
    });
    // i.define("stack-dump", |i: Handle| async move {
    //     println!("{:?}", Val::from(i.stack_get().await));
    // });
    i.define("call-stack", |mut i: Handle| async move {
        let cs = i.call_stack_names()
            .await
            .into_iter().map(|x| {
                if let Some(x) = x { Val::from(x) }
                else { false.into() }
            }).collect::<Vec<Val>>();
        i.stack_push(List::from(cs)).await;
    });
    i.define("stack-get", |mut i: Handle| async move {
        let s = i.stack_get().await;
        i.stack_push(s).await;
    });

    i.define("bool?", util::type_predicate::<bool>);
    i.define("bool-equal", util::equality::<bool>);
    i.define("symbol?", util::type_predicate::<Symbol>);
    i.define("symbol-equal", util::equality::<Symbol>);

    i.define("builtin?", util::type_predicate::<Builtin>);
    i.define("builtin-name", |mut i: Handle| async move {
        let b = i.stack_pop::<Builtin>().await;
        i.stack_push_option(Val::from(b).meta_ref().first_ref::<DefineMeta>()
                            .and_then(|m| m.name.as_ref()).map(|s| s.clone().to_symbol())).await;
    });

    i.define("value-set-error", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        i.stack_push(IsError::add(v)).await;
    });

    i.define("value-unset-error", |mut i: Handle| async move {
        let mut v = i.stack_pop_val().await;
        let m = v.meta_mut();
        // remove all errors
        while m.take_first::<IsError>().is_some() {}
        i.stack_push(v).await;
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
    i.define("enabled-features", move |mut i: Handle| {
        let enabled_features = enabled_features.clone();
        async move {
            i.stack_push(enabled_features.clone()).await;
        }
    });

}

