
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
    match e.downcast::<List<Val>>() {
        Ok(l) => i.eval(*l).await,
        Err(ee) => match ee.downcast::<Builtin>() {
            Ok(b) => i.eval(*b).await,
            Err(v) => {
                i.stack_push(v).await;
                i.stack_push("cannot eval").await;
                return i.pause().await;
            },
        },
    }
}

pub async fn command_line_arguments(mut i: Handle) {
    i.stack_push(List::from_vals(std::env::args())).await;
}

pub fn install(i: Builder) -> Builder {
    i.define("quote", quote)
        .define("drop", drop)
        .define("eval", eval)
        .define("command-line-arguments", command_line_arguments)
        .define("stack-dump", |mut i: Handle| async move {
            dbg!(i.stack_get().await);
        })
}

