
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

pub async fn eval(mut i: Handle) {
    let e =
        if let Some(e) = i.stack_pop_val().await { e }
        else {
            i.stack_push("stack-empty".to_symbol()).await;
            return i.pause().await;
        };
    match_downcast!(e, {
        l: List<Val> => i.eval(l).await,
        b: Builtin => i.eval(b).await,
        _ => {
            // i.stack_push(v).await;
            i.stack_push("cannot eval").await;
            return i.pause().await;
        }
    })
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

pub async fn command_line_arguments(mut i: Handle) {
    i.stack_push(List::from_vals(std::env::args())).await;
}

pub fn install(i: Builder) -> Builder {
    i.define("quote", quote)
        .define("drop", drop)
        .define("eval", eval)
        .define("const", const_)
        .define("upquote", upquote)
        .define("command-line-arguments", command_line_arguments)
        .define("stack-dump", |mut i: Handle| async move {
            dbg!(i.stack_get().await);
        })
}

