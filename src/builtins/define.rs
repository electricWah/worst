
use crate::base::*;
use crate::list::*;
use crate::interpreter::{Builtin, Builder, Handle};

async fn default_attributes(mut i: Handle) {
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
                    Ok(s) => (*s, List::default()), // name, no args
                    Err(ll) =>
                        match ll.downcast::<List<Val>>() {
                            Ok(l) =>
                                match i.quote().await {
                                    Some(qn) =>
                                        match qn.downcast::<Symbol>() {
                                            Ok(n) => (*n, *l), // name and args
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
                            Err(le) => {
                                i.stack_push(le).await;
                                i.stack_push("cannot define").await;
                                return i.pause().await;
                            },
                        },
                },
        };

    let body =
        match i.quote().await {
            Some(q) =>
                match q.downcast::<List<Val>>() {
                    Ok(l) => *l,
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

    println!("define {:?} {:?} {:?}", attrs, name, body);

    i.eval_child(attrs, |mut i: Handle| async move {
        // TODO default attributes
    }).await;

    i.define(name, body).await;
}

pub fn install(i: Builder) -> Builder {
    i.define("define", define)
        .define("default-attributes", default_attributes)
}

