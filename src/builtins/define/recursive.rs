
//! Recursive definition attribute

use crate::base::*;
use crate::interpreter::{Interpreter, Handle, Builtin};

/// Marker to prevent infinite recursion
#[derive(Clone)]
struct RecursiveCall;
impl Value for RecursiveCall {}

/// `define (recursive) infinite-loop [ infinite-loop ]`
// assuming no DefSet meta already, add one
// but also with an extra dynamic-ish binding to the same name
pub async fn recursive(mut i: Handle) {
    let name = i.stack_pop::<Symbol>().await;
    let mut body = i.stack_pop_val().await;
    let default = body.clone();

    let mut defs = i.all_definitions().await;
    let name_str = name.as_ref().to_string();
    defs.insert(name_str.clone(), Val::from(Builtin::from(move |mut i: Handle| {
        let name = name_str.clone();
        let default = default.clone();
        async move {
            match i.resolve_dynamic_where(name, |v| !v.meta_ref().contains::<RecursiveCall>()).await {
                Some(real) => i.eval(real).await,
                // eval this body by default
                // in case this definition is captured
                // and used elsewhere in a non-recursive context
                None => i.eval(default.clone()).await,
            }
        }
    })).with_meta(|m| m.push(RecursiveCall)));

    body.meta_ref_mut().push(defs);

    i.stack_push(body).await;
    i.stack_push(name).await;
}

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.define("recursive", recursive);
}

