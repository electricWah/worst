
//! Dynamic definition attribute

use crate::base::*;
use crate::interpreter::{Interpreter, Handle, Builtin, DefSet};

/// Marker to prevent infinite recursion
#[derive(Clone)]
struct Dynamic(Val);
impl Value for Dynamic {}

/// `define (dynamic) name [ body ... ]`
/// Define `name` to dynamically resolve by looking up the call stack
/// in local definitions only.
/// Thus, the most recent definition in a calling function is the one used.
/// Inconsistent use of this attribute on a definition may lead to surprising behaviour.
///
/// To avoid infinite recursion,
/// a [Dynamic] meta value is attached to the dynamic definition,
/// which contains the original body.
pub async fn dynamic(mut i: Handle) {
    let name = i.stack_pop::<Symbol>().await;
    let mut body: Val = i.stack_pop::<List>().await.into();

    let all_defs = i.all_definitions().await;
    DefSet::upsert_val(&mut body, |ds| ds.prepend(&all_defs));

    let dynamic_meta = Dynamic(body.clone());

    let body = {
        let name = name.as_ref().clone();
        Val::from(Builtin::from(move |mut i: Handle| {
            let name = name.clone();
            let default = body.clone();
            async move {
                match i.resolve_dynamic(name).await {
                    Some(d) => {
                        if let Some(Dynamic(real)) = d.meta_ref().first_ref::<Dynamic>() {
                            i.eval(real.clone()).await;
                        } else {
                            i.eval(d).await;
                        }
                    },
                    None => i.eval(default).await,
                }
            }
        })).with_meta(|m| m.push(dynamic_meta))
    };

    i.stack_push(body).await;
    i.stack_push(name).await;
}

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.define("dynamic", dynamic);
    i.define("dynamic-resolve", |mut i: Handle| async move {
        let name = i.stack_pop::<Symbol>().await.into_inner();
        let res = i.resolve_dynamic(name).await;
        i.stack_push_option(res).await;
    });
}

