
//! Basic documentation attribute and metadata

use crate::base::*;
use crate::impl_value;
use crate::interpreter::{Interpreter, Handle};

struct Doc(Val);
impl_value!(Doc);

/// Install all documentation functions.
pub fn install(i: &mut Interpreter) {
    i.define("value-doc-set", move |mut i: Handle| async move {
        let doc = i.stack_pop_val().await;
        let mut v = i.stack_pop_val().await;
        v.meta_ref_mut().push(Doc(doc));
        i.stack_push(v).await;
    });

    i.define("value-doc", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        let doc =
            if let Some(Doc(doc)) = v.meta_ref().first_ref::<Doc>() {
                doc.clone()
            } else {
                false.into()
            };
        i.stack_push(v).await;
        i.stack_push(doc).await;
    });
}


