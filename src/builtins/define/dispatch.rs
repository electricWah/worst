
//! Multimethods

use crate::base::*;
use crate::list::*;
use crate::interpreter::{Interpreter, Handle, Builtin, DefSet};

#[derive(Clone)]
struct DispatchInfo {
    clauses: Vec<(Val, Val)>, // (spec, body)
    default: Option<Val>,
}
impl Value for DispatchInfo {}

// FIXME def environment for each clause is rebound every define
// symptoms:
// - recursion can see later clauses
// - earlier clauses can break if they use imports no longer visible when
//   later clauses are added
// fix:
// - add DefineEnv to each clause body
// - add separate attribute to enable recursion
// - have it work somehow

async fn dispatch_impl(mut i: Handle, di: DispatchInfo) {
    for (spec, body) in di.clauses {
        i.eval(spec).await;
        if *i.stack_pop::<bool>().await.as_ref() {
            return i.eval(body).await;
        }
    }
    if let Some(default) = di.default {
        i.eval(default).await;
    } else {
        i.error(List::from(vec![Val::from("no-matching-dispatch".to_symbol())])).await;
    }
}

async fn dispatch_inner(mut i: Handle, first: bool) {
    let mut spec = i.quote_val().await;
    let name = i.stack_pop::<Symbol>().await;
    let body = i.stack_pop::<List>().await;
    let prev_def = i.resolve_definition(name.as_ref().clone()).await;
    let mut info =
        match prev_def {
            None => DispatchInfo { clauses: vec![], default: None },
            Some(prev) =>
                if let Some(di) = prev.meta_ref().first_ref::<DispatchInfo>() {
                    // modify previous DI
                    (*di).clone()
                } else {
                    // default is the non-dispatch version
                    DispatchInfo { clauses: vec![], default: Some(prev) }
                }
        };

    // add static def env
    let env = i.all_definitions().await;
    spec.meta_ref_mut().push(env.clone());
    let mut body = Val::from(body);
    if !body.meta_ref().contains::<DefSet>() {
        body.meta_ref_mut().push(env);
    }

    if first {
        info.clauses.insert(0, (spec, body));
    } else {
        info.clauses.push((spec, body));
    }

    // this could be better, perhaps some way of getting metadata for the
    // currently running definition
    let body_info = info.clone();
    let body = Val::from(Builtin::from(move |i: Handle| {
        let body_info = body_info.clone();
        async move {
            dispatch_impl(i, body_info.clone()).await
        }
    })).with_meta(|m| m.push(info));

    // i.stack_push(async move |mut i: Handle| move {
    // }).await;
    i.stack_push(body).await;
    i.stack_push(name).await;
}

/// `define (dispatch (predicate ...)) name [ body ... ]`
pub async fn dispatch(i: Handle) {
    dispatch_inner(i, false).await
}

/// `define (dispatch-first (predicate ...))
/// name [ body ... ]`
///
/// Usually, `stack-dispatch` puts new definitions at the end
/// (with the idea that dispatch predicates probably go from simple-and-common
/// to complex-and-rare as more are added).
/// Use this instead of `stack-dispatch`
/// to put this definition at the start instead.
pub async fn dispatch_first(i: Handle) {
    dispatch_inner(i, true).await
}

/// Check the stack conforms to a list of predicates.
/// `[pred pred...] stack-matches?`
///
/// The rightmost predicate (the end of the list) is the top of the stack.
pub async fn stack_matches(mut i: Handle) {
    let mut preds = i.stack_pop::<List>().await.into_inner();
    let stack = i.stack_get().await;
    if preds.len() > stack.len() {
        return i.stack_push(false).await;
    }
    preds.reverse();
    for (pred, item) in preds.zip(stack) {
        i.stack_push(item).await;
        if let Some(v) = pred.downcast_ref::<Symbol>() {
            i.call(v.clone()).await;
        } else {
            i.eval(pred).await;
        }
        let ok = i.stack_pop::<bool>().await.into_inner();
        let _item = i.stack_pop_val().await;
        if !ok {
            return i.stack_push(false).await;
        }
    }
    i.stack_push(true).await;
}

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.define("dispatch", dispatch);
    i.define("dispatch-first", dispatch_first);

    // maybe builtins/stack.rs
    i.define("stack-matches?", stack_matches);
}

