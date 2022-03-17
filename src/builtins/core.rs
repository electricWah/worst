
use match_downcast::*;
use crate::base::*;
use crate::list::*;
use crate::interpreter::{Builtin, Builder, Handle};

pub async fn quote(mut i: Handle) {
    if let Some(q) = i.quote().await {
        i.stack_push(q).await;
    } else {
        i.stack_push("quote-nothing".to_symbol()).await;
        return i.pause().await;
    }
}

pub async fn drop(mut i: Handle) {
    if i.stack_pop_val().await.is_none() {
        i.stack_push("stack-empty".to_symbol()).await;
        return i.pause().await;
    }
}

pub async fn dig(mut i: Handle) {
    let a = i.stack_pop_val().await;
    let b = i.stack_pop_val().await;
    let c = i.stack_pop_val().await;
    match (a, b, c) {
        (Some(a), Some(b), Some(c)) => {
            i.stack_push(b).await;
            i.stack_push(a).await;
            i.stack_push(c).await;
        },
        _ => {
            i.stack_push("stack-empty".to_symbol()).await;
            return i.pause().await;
        },
    }
}

pub async fn bury(mut i: Handle) {
    let a = i.stack_pop_val().await;
    let b = i.stack_pop_val().await;
    let c = i.stack_pop_val().await;
    match (a, b, c) {
        (Some(a), Some(b), Some(c)) => {
            i.stack_push(a).await;
            i.stack_push(c).await;
            i.stack_push(b).await;
        },
        _ => {
            i.stack_push("stack-empty".to_symbol()).await;
            return i.pause().await;
        },
    }
}

pub async fn equal(mut i: Handle) {
    let a = i.stack_pop_val().await;
    let b = i.stack_pop_val().await;
    match (a, b) {
        (Some(a), Some(b)) => {
            let eq = a.equal(&b);
            i.stack_push(b).await;
            i.stack_push(a).await;
            i.stack_push(eq).await;
        },
        _ => {
            i.stack_push("stack-empty".to_symbol()).await;
            return i.pause().await;
        },
    }
}

pub async fn false_(mut i: Handle) {
    if let Some(v) = i.stack_pop_val().await {
        let is = Some(&false) == v.downcast_ref::<bool>();
        i.stack_push(v).await;
        i.stack_push(is).await;
    } else {
        i.stack_push("stack-empty".to_symbol()).await;
        return i.pause().await;
    }
}

pub async fn eval(mut i: Handle) {
    let e =
        if let Some(e) = i.stack_pop_val().await { e }
        else {
            i.stack_push("stack-empty".to_symbol()).await;
            return i.pause().await;
        };
    i.eval(e).await;
}

pub async fn call(mut i: Handle) {
    if let Some(c) = i.stack_pop::<Symbol>().await {
        i.call(c).await;
    } else {
        i.stack_push("stack-empty".to_symbol()).await;
        return i.pause().await;
    }
}

/// define const [
///     [quote] swap list-push list-reverse
///     upquote
///     quote definition-add uplevel
/// ]
pub async fn const_(mut i: Handle) {
    let v =
        if let Some(e) = i.stack_pop_val().await { e }
        else {
            i.stack_push("stack-empty".to_symbol()).await;
            return i.pause().await;
        };
    let name =
        if let Some(q) = i.quote().await {
            match q.downcast::<Symbol>() {
                Ok(n) => n,
                Err(qq) => {
                    i.stack_push(qq).await;
                    i.stack_push("const: not a symbol").await;
                    return i.pause().await;
                },
            }
        }
        else {
            i.stack_push("quote-nothing".to_symbol()).await;
            return i.pause().await;
        };

    i.define(name.as_string(), move |mut i: Handle| {
        let vv = v.clone();
        async move {
            i.stack_push(vv.clone()).await;
        }
    }).await;
}

/// [ quote quote quote uplevel uplevel ] quote upquote definition-add
pub async fn upquote(mut i: Handle) {
    i.uplevel(|mut i: Handle| async move {
        if let Some(q) = i.quote().await {
            i.stack_push(q).await;
        }
        else {
            i.stack_push("quote-nothing".to_symbol()).await;
            return i.pause().await;
        }
    }).await;
}

pub async fn swap(mut i: Handle) {
    let a = i.stack_pop_val().await;
    let b = i.stack_pop_val().await;
    match (a, b) {
        (Some(a), Some(b)) => {
            i.stack_push(a).await;
            i.stack_push(b).await;
        },
        _ => {
            i.stack_push("stack-empty".to_symbol()).await;
            return i.pause().await;
        }
    }
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
    if let Some(cond) = i.quote().await {
        if let Some(body) = i.quote().await {
            loop {
                i.eval(cond.clone()).await;
                if i.stack_pop::<bool>().await != Some(true) { break; }
                i.eval(body.clone()).await;
            }
        }
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
    if let Some(ift) = i.quote().await {
        if let Some(iff) = i.quote().await {
            if let Some(false) = i.stack_pop::<bool>().await {
                i.eval(iff).await;
            } else {
                i.eval(ift).await;
            }
        }
    }
}

pub async fn command_line_arguments(mut i: Handle) {
    i.stack_push(List::from_vals(std::env::args())).await;
}

pub async fn print(mut i: Handle) {
    if let Some(s) = i.stack_pop::<String>().await {
        print!("{}", s);
    } // else?
}

pub async fn add(mut i: Handle) {
    let a = i.stack_pop::<i32>().await;
    let b = i.stack_pop::<i32>().await;
    match (a, b) {
        (Some(a), Some(b)) => i.stack_push(a + b).await,
        _ => {
            i.stack_push("coould'nt add").await;
            i.pause().await;
        }
    }
}

pub fn install(mut i: Builder) -> Builder {
    i.define("quote", quote);
    i.define("drop", drop);
    i.define("dig", dig);
    i.define("bury", bury);
    i.define("eval", eval);
    i.define("call", call);
    i.define("const", const_);
    i.define("upquote", upquote);
    i.define("swap", swap);
    i.define("if", if_);
    i.define("while", while_);
    i.define("equal?", equal);
    i.define("false?", false_);
    i.define("command-line-arguments", command_line_arguments);
    i.define("add", add);
    i.define("print", print); // temporary
    i.define("stack-dump", |mut i: Handle| async move {
        dbg!(i.stack_get().await);
    });
    i
}

