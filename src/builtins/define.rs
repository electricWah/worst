
use crate::base::*;
use crate::list::*;
use crate::interpreter::{Builder, Handle};

async fn default_attributes(mut _i: Handle) {
    // TODO into_new_frame
}

/// define (attributes) name [ body ... ]
pub async fn define(mut i: Handle) {

    let (name, attrs) =
        match i.quote().await {
            None => {
                i.stack_push("define: quote-nothing").await;
                return i.pause().await;
            }
            Some(na) =>
                match na.downcast::<Symbol>() {
                    Ok(s) => (s, List::default()),
                    Err(e) =>
                        match e.downcast::<List>() {
                            Ok(l) =>
                                match i.quote().await {
                                    Some(qn) =>
                                        match qn.downcast::<Symbol>() {
                                            Ok(n) => (n, l), // name and args
                                            Err(qe) => {
                                                i.stack_push(qe).await;
                                                i.stack_push("define: expected symbol").await;
                                                return i.pause().await;
                                            }
                                        },
                                    None => {
                                        i.stack_push("define: quote-nothing").await;
                                        return i.pause().await;
                                    },
                                },
                            Err(_) => {
                                // i.stack_push(le).await;
                                i.stack_push("cannot define").await;
                                return i.pause().await;
                            }
                        }
                }
        };

    let body =
        match i.quote().await {
            Some(q) =>
                match q.downcast::<List>() {
                    Ok(l) => l,
                    Err(e) => {
                        i.stack_push(e).await;
                        i.stack_push("define: expected list").await;
                        return i.pause().await;
                    },
                },
            None => {
                i.stack_push("define: no body").await;
                return i.pause().await;
            },
        };

    // println!("define {:?} {:?} {:?}", attrs, name, body);

    i.eval_child(attrs, |mut _i: Handle| async move {
        // TODO default attributes
    }).await;

    let env = i.all_definitions().await;

    i.define_closure(name, body, env).await;
}

pub fn install(mut i: Builder) -> Builder {
    i.define("define", define);
    i.define("default-attributes", default_attributes);
    i.define("definition-add", |mut i: Handle| async move {
        let name = i.stack_pop::<Symbol>().await;
        let def = i.stack_pop_val().await;
        i.define(name, def).await;
    });
    i.define("all-definitions", |mut i: Handle| async move {
        let p = i.all_definitions().await;
        i.stack_push(List::from_pairs(p.iter().map(|(k, v)| (k.to_symbol(), v.clone())))).await;
    });
    i.define("definition-resolve", |mut i: Handle| async move {
        let name = i.stack_pop::<Symbol>().await;
        let res = i.resolve_definition(name.clone()).await;
        i.stack_push(name).await;
        match res {
            Some(def) => i.stack_push(def).await,
            None => i.stack_push(false).await,
        }
    });
    i
    // local name = i:stack_pop(Symbol)
    // local body = i:stack_pop({List, "function"})
    // local interp = i:stack_ref(1, Interpreter)
    // interp:define(name, body)
}

