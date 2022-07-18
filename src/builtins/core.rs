
//! Basic stack-shuffling and control flow builtins

use crate::base::*;
use crate::list::*;
use crate::interpreter::{Interpreter, Handle};

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

/// `equal` - Compare the top two values on the stack for equality and
/// a bool on top of the stack (true if they are equal, false otherwise).
pub async fn equal(mut i: Handle) {
    let a = i.stack_pop_val().await;
    let b = i.stack_pop_val().await;
    let eq = a == b;
    i.stack_push(b).await;
    i.stack_push(a).await;
    i.stack_push(eq).await;
}

/// `false?` - Put true on top of the stack if the top value on the stack
/// is false, and false otherwise. This is awkward wording.
pub async fn false_(mut i: Handle) {
    let v = i.stack_top_val().await;
    let is = Some(&false) == v.downcast_ref::<bool>();
    i.stack_push(is).await;
}

/// `not` - Remove the value on top of the stack and replace it with true
/// if it was false (and false otherwise).
pub async fn not(mut i: Handle) {
    let v = i.stack_pop_val().await;
    let is = Some(&false) == v.downcast_ref::<bool>();
    i.stack_push(is).await;
}

/// `error?` - Check if the value on top of the stack is an error with
/// [IsError::is_error], and put its result on top of the stack.
pub async fn error_(mut i: Handle) {
    let v = i.stack_top_val().await;
    let is = IsError::is_error(&v);
    i.stack_push(is).await;
}

/// `eval` - Evaluate the value on top of the stack. See [Handle::eval].
pub async fn eval(mut i: Handle) {
    let e = i.stack_pop_val().await;
    i.eval(e).await;
}

/// `call` - Call the value on top of the stack. See [Handle::call].
pub async fn call(mut i: Handle) {
    let c = i.stack_pop::<Symbol>().await;
    i.call(c.into_inner()).await;
}

/// `uplevel` - Call the value on top of the stack as if in the parent stack frame.
pub async fn uplevel(mut i: Handle) {
    i.uplevel(|mut i: Handle| async move {
        let c = i.stack_pop::<Symbol>().await;
        i.call(c.into_inner()).await;
    }).await;
}

/// `const` - Awkward `let`
/// ```
/// define const [
///     [quote] swap list-push list-reverse
///     upquote
///     quote definition-add uplevel
/// ]
/// ```
pub async fn const_(mut i: Handle) {
    let v = i.stack_pop_val().await;
    let qname = i.quote_val().await;
    let name =
        // TODO quote_ty::<Symbol>()
        if let Some(v) = qname.downcast_ref::<Symbol>() {
            v
        } else {
            return i.error(List::from(vec![
                "const: not a symbol".to_string().into(),
                    qname,
            ])).await;
        };

    i.define(name.as_ref(), move |mut i: Handle| {
        let vv = v.clone();
        async move {
            i.stack_push(vv.clone()).await;
        }
    }).await;
}

/// `[ quote quote quote uplevel uplevel ] quote upquote definition-add`
pub async fn upquote(mut i: Handle) {
    i.uplevel(|mut i: Handle| async move {
        let q = i.quote_val().await;
        i.stack_push(q).await;
    }).await;
}

/// `while` - Do things until don't
/// ```
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
/// ```
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

/// `add` - Add the top two numbers on the stack and replace them with their sum.
pub async fn add(mut i: Handle) {
    let a = i.stack_pop::<i32>().await.into_inner();
    let b = i.stack_pop::<i32>().await.into_inner();
    i.stack_push(a + b).await;
}

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.define("quote", quote);
    i.define("clone", clone);
    i.define("drop", drop);
    i.define("dig", dig);
    i.define("bury", bury);
    i.define("eval", eval);
    i.define("call", call);
    i.define("uplevel", uplevel);
    i.define("upquote", upquote);
    i.define("const", const_);
    i.define("swap", swap);
    i.define("if", if_);
    i.define("while", while_);
    i.define("equal?", equal);
    i.define("not", not);
    i.define("false?", false_);
    i.define("error?", error_);
    i.define("pause", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        i.pause(v).await;
    });
    i.define("add", add);
    i.define("stack-empty", |mut i: Handle| async move {
        let v = i.stack_empty().await;
        i.stack_push(v).await;
    });
    i.define("stack-dump", |i: Handle| async move {
        println!("{:?}", Val::from(i.stack_get().await));
    });
    // for now
    i.define("value->string", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        i.stack_push(format!("{:?}", v)).await;
    });
    i.define("stack-get", |mut i: Handle| async move {
        let s = i.stack_get().await;
        i.stack_push(s).await;
    });
}

