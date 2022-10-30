
//! `define` and other definition-related builtins

use crate::base::*;
use crate::interpreter::{Interpreter, Handle, DefineMeta, DefSet};

mod dispatch;
mod dynamic;
mod recursive;

async fn default_attributes(mut _i: Handle) {
    // TODO into_new_frame
}

/// define (attributes) name [ body ... ]
pub async fn define(mut i: Handle) {

    let (name, attrs) = {
        let q = i.quote_val().await;
        if let Some(name) = q.downcast_ref::<Symbol>() {
            (name.clone(), List::default())
        } else if let Some(attrs) = q.downcast_ref::<List>() {
            let qname = i.quote_val().await;
            if let Some(name) = qname.downcast_ref::<Symbol>() {
                (name.clone(), attrs.clone())
            } else {
                return i.error(List::from(vec![
                    "define: expected symbol".to_string().into(),
                    qname,
                ])).await;
            }
        } else {
            return i.error("cannot define".to_string()).await;
        }
    };

    let body =
        if let Some(l) = i.quote_val().await.downcast::<List>() {
            l
        } else {
            return i.error(List::from(vec![
                "define: expected list".to_string().into(),
            ])).await;
        };

    i.stack_push(body).await;
    i.stack_push(name.clone()).await;

    if !attrs.is_empty() {
        i.eval_child(attrs.clone(), move |mut i: Handle| async move {
            i.add_definition("definition-attributes", true).await;
        }).await;
    }
    i.call("default-attributes").await;

    let name = i.stack_pop::<Symbol>().await.into_inner();
    let mut body = i.stack_pop_val().await;

    if !body.meta_ref().contains::<DefineMeta>() {
        body.meta_ref_mut().push(DefineMeta { name: Some(name.clone().to_string()) });
    }
    if !body.meta_ref().contains::<DefSet>() {
        body.meta_ref_mut().push(i.all_definitions().await);
    }

    i.add_definition(name, body).await;
}

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.add_definition("definition-attributes", false);
    i.define("define", define);
    i.define("default-attributes", default_attributes);
    i.define("definition-add", |mut i: Handle| async move {
        let name = i.stack_pop::<Symbol>().await.into_inner();
        let def = i.stack_pop_val().await;
        i.define(name, def).await;
    });
    i.define("all-definitions", |mut i: Handle| async move {
        let p = i.all_definitions().await;
        i.stack_push(List::from_pairs(p.iter().map(|(k, v)| (k.to_symbol(), v.clone())))).await;
    });
    i.define("definition-resolve", |mut i: Handle| async move {
        let name = i.stack_pop::<Symbol>().await.into_inner();
        let res = i.resolve_definition(name.clone()).await;
        i.stack_push(name).await;
        match res {
            Some(def) => i.stack_push(def).await,
            None => i.stack_push(false).await,
        }
    });
    dispatch::install(i);
    dynamic::install(i);
    recursive::install(i);
}

