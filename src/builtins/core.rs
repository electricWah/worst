
use crate::base::*;
use crate::list::*;
use crate::interpreter::{Interpreter, Handle};

pub async fn quote(mut i: Handle) {
    let q = i.quote_val().await;
    i.stack_push(q).await;
}

pub async fn drop(mut i: Handle) {
    i.stack_pop_val().await;
}

pub async fn clone(mut i: Handle) {
    let v = i.stack_pop_val().await;
    i.stack_push(v.clone()).await;
    i.stack_push(v).await;
}

pub async fn swap(mut i: Handle) {
    let a = i.stack_pop_val().await;
    let b = i.stack_pop_val().await;
    i.stack_push(a).await;
    i.stack_push(b).await;
}

pub async fn dig(mut i: Handle) {
    let a = i.stack_pop_val().await;
    let b = i.stack_pop_val().await;
    let c = i.stack_pop_val().await;
    i.stack_push(b).await;
    i.stack_push(a).await;
    i.stack_push(c).await;
}

pub async fn bury(mut i: Handle) {
    let a = i.stack_pop_val().await;
    let b = i.stack_pop_val().await;
    let c = i.stack_pop_val().await;
    i.stack_push(a).await;
    i.stack_push(c).await;
    i.stack_push(b).await;
}

pub async fn equal(mut i: Handle) {
    let a = i.stack_pop_val().await;
    let b = i.stack_pop_val().await;
    let eq = a == b;
    i.stack_push(b).await;
    i.stack_push(a).await;
    i.stack_push(eq).await;
}

pub async fn false_(mut i: Handle) {
    let v = i.stack_top_val().await;
    let is = Some(&false) == v.downcast_ref::<bool>();
    i.stack_push(is).await;
}

pub async fn not(mut i: Handle) {
    let v = i.stack_pop_val().await;
    let is = Some(&false) == v.downcast_ref::<bool>();
    i.stack_push(is).await;
}

pub async fn error(mut i: Handle) {
    let v = i.stack_top_val().await;
    let is = IsError::is_error(&v);
    i.stack_push(is).await;
}

pub async fn eval(mut i: Handle) {
    let e = i.stack_pop_val().await;
    i.eval(e).await;
}

pub async fn call(mut i: Handle) {
    let c = i.stack_pop::<Symbol>().await;
    i.call(c.into_inner()).await;
}

pub async fn uplevel(mut i: Handle) {
    i.uplevel(|mut i: Handle| async move {
        let c = i.stack_pop::<Symbol>().await;
        i.call(c.into_inner()).await;
    }).await;
}

/// define const [
///     [quote] swap list-push list-reverse
///     upquote
///     quote definition-add uplevel
/// ]
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

/// [ quote quote quote uplevel uplevel ] quote upquote definition-add
pub async fn upquote(mut i: Handle) {
    i.uplevel(|mut i: Handle| async move {
        let q = i.quote_val().await;
        i.stack_push(q).await;
    }).await;
}

/// ; while [-> bool] [body ...]
/// define while [
///     upquote quote %%cond definition-add
///     upquote quote %%while-body definition-add
///     [
///         %%cond if [%%while-body %%loop] [[]] current-context-set-code
///     ] const %%loop
///     %%loop current-context-set-code
/// ]
pub async fn while_(mut i: Handle) {
    let cond = i.quote_val().await;
    let body = i.quote_val().await;
    loop {
        i.eval(cond.clone()).await;
        if !i.stack_pop::<bool>().await.into_inner() { break; }
        i.eval(body.clone()).await;
    }
}

/// ; bool if [if-true] [if-false]
/// define if [
///     upquote upquote
///     ; cond true false => false true cond
///     swap dig
///     quote swap when drop
///     quote eval uplevel
/// ]
pub async fn if_(mut i: Handle) {
    let ift = i.quote_val().await;
    let iff = i.quote_val().await;
    if i.stack_pop::<bool>().await.into_inner() {
        i.eval(ift).await;
    } else {
        i.eval(iff).await;
    }
}

pub async fn command_line_arguments(mut i: Handle) {
    i.stack_push(List::from_iter(std::env::args())).await;
}

pub async fn add(mut i: Handle) {
    let a = i.stack_pop::<i32>().await.into_inner();
    let b = i.stack_pop::<i32>().await.into_inner();
    i.stack_push(a + b).await;
}

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
    i.define("error?", error);
    i.define("pause", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        i.pause(v).await;
    });
    i.define("command-line-arguments", command_line_arguments);
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

