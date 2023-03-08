
//! Basic documentation attribute and metadata

use crate::base::*;
use crate::interpreter::*;

// TODO use normal meta-entry stuff and a list (possibly unique) for this

struct Doc(Val);
impl Value for Doc {}

/// Install all documentation functions.
pub fn install(i: &mut Interpreter) {
    i.add_builtin("value-doc-set", |i: &mut Interpreter| {
        let doc = i.stack_pop_val()?;
        let mut v = i.stack_pop_val()?;
        v.meta_mut().insert(Doc(doc));
        i.stack_push(v);
        Ok(())
    });

    i.add_builtin("value-doc", |i: &mut Interpreter| {
        let v = i.stack_pop_val()?;
        let doc =
            if let Some(Doc(doc)) = v.meta_ref().get_ref::<Doc>() {
                doc.clone()
            } else {
                false.into()
            };
        i.stack_push(v);
        i.stack_push(doc);
        Ok(())
    });
}

