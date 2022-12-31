
//! Conjuring, manipulating and executing interpreters

use std::rc::Rc;
use std::cell::RefCell;

use crate::base::*;
use crate::interpreter::{Interpreter, Handle, DefScope};

// TODO no wrapper, just use Interpreter directly and wrap in a place in worst
#[derive(Clone, Default)]
struct Interp(Rc<RefCell<Interpreter>>);
impl Value for Interp {}

/// Install all the interpreter functions.
pub fn install(i: &mut Interpreter) {
    i.define("interpreter-empty", |mut i: Handle| async move {
        i.stack_push(Interp::default()).await;
    });
    i.define("interpreter-run",  |mut i: Handle| async move {
        let interp = i.stack_top::<Interp>().await;
        let r = interp.as_ref().0.borrow_mut().run();
        match r {
            None => i.stack_push(true).await,
            Some(e) => {
                i.stack_push(e).await;
                i.stack_push(false).await;
            },
        }
    });
    i.define("interpreter-reset",  |i: Handle| async move {
        let interp = i.stack_top::<Interp>().await;
        interp.as_ref().0.borrow_mut().reset();
    });
    i.define("interpreter-stack-length",  |mut i: Handle| async move {
        let interp = i.stack_top::<Interp>().await;
        let len = interp.as_ref().0.borrow().stack_ref().len();
        i.stack_push(len as i64).await;
    });
    i.define("interpreter-stack-push",  |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        let interp = i.stack_top::<Interp>().await;
        interp.as_ref().0.borrow_mut().stack_push(v);
    });
    i.define("interpreter-stack-pop",  |mut i: Handle| async move {
        let interp = i.stack_top::<Interp>().await;
        let v = interp.as_ref().0.borrow_mut().stack_ref_mut().pop().unwrap_or_else(|| false.into());
        i.stack_push(v).await;
    });
    i.define("interpreter-stack-get",  |mut i: Handle| async move {
        let interp = i.stack_top::<Interp>().await;
        let s = interp.as_ref().0.borrow_mut().stack_ref().clone();
        i.stack_push(s).await;
    });
    i.define("interpreter-definition-add", |mut i: Handle| async move {
        let def = i.stack_pop_val().await;
        let name = i.stack_pop::<Symbol>().await.into_inner();
        let interp = i.stack_top::<Interp>().await;
        interp.as_ref().0.borrow_mut().add_definition(name, def, DefScope::Local);
    });
    i.define("interpreter-definition-remove", |mut i: Handle| async move {
        let name = i.stack_pop::<Symbol>().await;
        let interp = i.stack_top::<Interp>().await;
        interp.as_ref().0.borrow_mut().definition_remove(name.as_ref());
    });
    i.define("interpreter-eval-next", |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        let interp = i.stack_top::<Interp>().await;
        interp.as_ref().0.borrow_mut().eval_next(v);
    });
    i.define("interpreter-body-push",  |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        let interp = i.stack_top::<Interp>().await;
        interp.as_ref().0.borrow_mut().body_mut().push(v);
    });
    i.define("interpreter-body-prepend",  |mut i: Handle| async move {
        let body = i.stack_pop::<List>().await.into_inner();
        let interp = i.stack_pop::<Interp>().await;
        interp.as_ref().0.borrow_mut().body_mut().prepend(body);
        i.stack_push(interp).await;
    });
}

