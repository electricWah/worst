
//! Unique values are only equal to themselves and their clones.

use std::rc::Rc;
use std::cell::Cell;
use crate::base::*;
use super::util;
use crate::interpreter::*;

#[derive(PartialEq, Eq)]
struct Unique(u64);
impl Value for Unique {}

/// Install some string functions.
pub fn install(i: &mut Interpreter) {
    util::add_type_predicate_builtin::<Unique>(i, "unique?");
    i.add_builtin("unique-equal", util::equality::<Unique>);

    let gensym = Rc::new(Cell::new(0u64));
    i.add_builtin("make-unique", move |i: &mut Interpreter| {
        let gensym = gensym.clone();
        let g = gensym.get();
        gensym.set(g + 1);
        i.stack_push(Unique(g));
        Ok(())
    });
}

