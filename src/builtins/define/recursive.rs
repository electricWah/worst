
//! Recursive definition attribute

use async_recursion::async_recursion;
use crate::base::*;
use crate::interpreter::{Interpreter, Handle, Builtin, DefSet};

/// Marker to prevent infinite recursion
#[derive(Clone)]
struct RecursiveCall;
impl Value for RecursiveCall {}

#[async_recursion(?Send)]
async fn resolve_dynamic_recursive(mut i: Handle, name: String, default: Val) {
    if let Some(def) = i.local_definition(name.clone()).await {
        if !def.meta_ref().contains::<RecursiveCall>() {
            return i.stack_push(def).await;
        }
    }
    if i.is_toplevel().await {
        i.stack_push(default).await;
    } else {
        i.uplevel(move |i: Handle| async move {
            resolve_dynamic_recursive(i, name, default).await;
        }).await;
    }
}

/// `define (recursive) infinite-loop [ infinite-loop ]`
// assuming no DefSet meta already, add one
// but also with an extra dynamic-ish binding to the same name
pub async fn recursive(mut i: Handle) {
    let name = i.stack_pop::<Symbol>().await;
    let mut body = i.stack_pop_val().await;
    let default = body.clone();

    let name_str = name.as_ref().to_string();
    DefSet::upsert_val(&mut body, |ds| {
        ds.insert(name_str.clone(), Val::from(Builtin::from(move |mut i: Handle| {
            let name = name_str.clone();
            let default = default.clone();
            async move {
                i.eval(move |i: Handle| async move {
                    resolve_dynamic_recursive(i, name, default).await;
                }).await;
                let res = i.stack_pop_val().await;
                i.eval(res).await;
            }
        })).with_meta(|m| m.push(RecursiveCall)));
    });

    i.stack_push(body).await;
    i.stack_push(name).await;
}

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.define("recursive", recursive);
}

