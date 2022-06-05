
use crate::base::*;
use crate::interpreter::{Interpreter, Handle};

pub fn install(i: &mut Interpreter) {
    i.define("string?", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        let is = v.is::<String>();
        i.stack_push(v).await;
        i.stack_push(is).await;
    });
    i.define("string-append", |mut i: Handle| async move {
        let b = i.stack_pop::<String>().await;
        let mut a = i.stack_pop::<String>().await;
        a.push_str(&b);
        i.stack_push(a).await;
    });
    i.define("whitespace?", |mut i:Handle| async move {
        let s = i.stack_pop::<String>().await;
        let ws = s.chars().all(char::is_whitespace);
        i.stack_push(s).await;
        i.stack_push(ws).await;
    });
    i.define("string->symbol", |mut i: Handle| async move {
        let s = i.stack_pop::<String>().await;
        i.stack_push(s.to_symbol()).await;
    });
}

