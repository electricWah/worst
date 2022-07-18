
//! Basic documentation attribute and metadata

use crate::base::*;
use crate::impl_value;
use crate::interpreter::{Interpreter, Handle};

struct Doc(Val);
impl_value!(Doc);

/// Install all documentation functions.
pub fn install(i: &mut Interpreter) {
    let doc_place = Place::wrap(false);
    let doc_place_set = doc_place.clone();
    let doc_place_take = doc_place;

    i.define("doc", move |mut i: Handle| {
        let mut doc_place = doc_place_set.clone();
        async move {
            let docs = i.quote_val().await;
            doc_place.set(docs);
        }
    });

    i.define("documentation-attribute", move |mut i: Handle| {
        let mut doc_place = doc_place_take.clone();
        async move {
            let docs = doc_place.swap(false);
            if docs.downcast_ref::<bool>() != Some(&false) {
                let name = i.stack_pop_val().await;
                let mut body = i.stack_pop_val().await;
                body.meta_ref_mut().push(Doc(docs));
                i.stack_push(body).await;
                i.stack_push(name).await;
            }
        }
    });

    i.define("value-doc", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        let doc =
            if let Some(Doc(doc)) = v.meta_ref().first::<Doc>() {
                doc.clone()
            } else {
                false.into()
            };
        i.stack_push(v).await;
        i.stack_push(doc).await;
    });
}


