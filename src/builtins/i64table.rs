
//! [I64Table](crate::data::i64table::I64Table) manipulation basics

use crate::base::*;
use crate::data::i64table::*;
use crate::builtins::util;
use crate::interpreter::{Interpreter, Handle};

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    i.define("make-i64table", util::make_default::<I64Table>);
    i.define("i64table?", util::type_predicate::<I64Table>);
    i.define("i64table-hash-insert", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        let k = i.stack_pop::<ValueHash>().await.into_inner();
        let mut ht = i.stack_pop::<I64Table>().await;
        ht.as_mut().insert(k, v);
        i.stack_push(ht).await;
    });
    i.define("i64table-hash-keys", |mut i: Handle| async move {
        let ht = i.stack_pop::<I64Table>().await;
        i.stack_push(List::from(ht.as_ref().key_hashes().cloned().map(Val::from).collect::<Vec<_>>())).await;
    });
    i.define("i64table-hash-get", |mut i: Handle| async move {
        let hash = i.stack_pop::<ValueHash>().await;
        let ht = i.stack_pop::<I64Table>().await;
        match ht.as_ref().key_pairs(hash.as_ref()) {
            None => i.stack_push(List::default()).await,
            Some(pairs) => {
                i.stack_push(List::from_pairs(pairs.iter().cloned())).await;
            },
        }

    });
}


