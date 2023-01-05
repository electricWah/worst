
//! Strings (of utf8 characters)

use crate::base::*;
use super::util;
use crate::interpreter::{Interpreter, Handle};

/// Install some string functions.
pub fn install(i: &mut Interpreter) {
    i.define("string?", util::type_predicate::<String>);
    i.define("string-equal", util::equality::<String>);
    i.define("string-compare", util::comparison::<String>);
    // i.define("string-hash", util::value_hash::<String>);
    i.define("string-append", |mut i: Handle| async move {
        let b = i.stack_pop::<String>().await;
        let mut a = i.stack_pop::<String>().await;
        a.as_mut().push_str(b.as_ref());
        i.stack_push(a).await;
    });
    i.define("string-split", |mut i: Handle| async move {
        let p = i.stack_pop::<String>().await;
        let s = i.stack_pop::<String>().await;
        let split = s.as_ref().split(p.as_ref()).map(String::from);
        i.stack_push(List::from_iter(split)).await;
    });
    i.define("whitespace?", |mut i:Handle| async move {
        let s = i.stack_top::<String>().await;
        let ws = s.as_ref().chars().all(char::is_whitespace);
        i.stack_push(ws).await;
    });
    i.define("string->symbol", |mut i: Handle| async move {
        let s = i.stack_pop::<String>().await;
        i.stack_push(s.into_inner().to_symbol()).await;
    });
    i.define("symbol->string", |mut i: Handle| async move {
        let s = i.stack_pop::<Symbol>().await;
        i.stack_push(s.into_inner().to_string()).await;
    });
}

