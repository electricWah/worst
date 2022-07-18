
//! Conjuring, manipulating and executing interpreters

use std::rc::Rc;
use std::cell::RefCell;

use crate::impl_value;
use crate::base::*;
use crate::list::*;
use crate::interpreter::{Interpreter, Handle};

// TODO no wrapper, just use Interpreter directly
#[derive(Clone)]
struct Interp(Rc<RefCell<Interpreter>>);

impl PartialEq for Interp {
    fn eq(&self, other: &Interp) -> bool { Rc::ptr_eq(&self.0, &other.0) }
}
impl Eq for Interp {}
impl_value!(Interp, type_name("Interpreter"));

impl Interp {
    fn define(&self, name: impl Into<String>, def: Val) {
        self.0.borrow_mut().define(name, def);
    }
    fn call(&self, name: impl Into<Symbol>) {
        self.0.borrow_mut().call(name);
    }
    fn run(&self) -> Option<Val> {
        self.0.borrow_mut().run()
    }
    fn body_prepend(&self, body: List) {
        self.0.borrow_mut().body_mut().prepend(body);
    }
}

impl Default for Interp {
    fn default() -> Self {
        Interp(Rc::new(RefCell::new(Interpreter::default())))
    }
}

/// Install all the interpreter functions.
pub fn install(i: &mut Interpreter) {
    i.define("interpreter-empty", |mut i: Handle| async move {
        i.stack_push(Interp::default()).await;
    });
    i.define("interpreter-run",  |mut i: Handle| async move {
        let interp = i.stack_pop::<Interp>().await;
        let r = interp.as_ref().run();
        i.stack_push(interp).await;
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
        let len = interp.as_ref().0.borrow_mut().stack_len();
        i.stack_push(len as i32).await;
    });
    i.define("interpreter-stack-push",  |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        let interp = i.stack_top::<Interp>().await;
        interp.as_ref().0.borrow_mut().stack_push(v);
    });
    i.define("interpreter-stack-pop",  |mut i: Handle| async move {
        let interp = i.stack_top::<Interp>().await;
        let v = interp.as_ref().0.borrow_mut().stack_pop_val().unwrap_or_else(|| false.into());
        i.stack_push(v).await;
    });
    i.define("interpreter-stack-get",  |mut i: Handle| async move {
        let interp = i.stack_top::<Interp>().await;
        let s = interp.as_ref().0.borrow_mut().stack_ref().clone();
        i.stack_push(s).await;
    });
    i.define("interpreter-definition-add", |mut i: Handle| async move {
        let name = i.stack_pop::<Symbol>().await.into_inner();
        let def = i.stack_pop_val().await;
        let interp = i.stack_top::<Interp>().await;
        interp.as_ref().define(name, def);
    });
    i.define("interpreter-definition-remove", |mut i: Handle| async move {
        let name = i.stack_pop::<Symbol>().await;
        let interp = i.stack_top::<Interp>().await;
        interp.as_ref().0.borrow_mut().definition_remove(name.as_ref());
    });
    i.define("interpreter-call", |mut i: Handle| async move {
        let name = i.stack_pop::<Symbol>().await.into_inner();
        let interp = i.stack_top::<Interp>().await;
        interp.as_ref().call(name);
    });
    i.define("interpreter-body-push",  |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        let interp = i.stack_top::<Interp>().await;
        interp.as_ref().0.borrow_mut().body_mut().push(v);
    });
    i.define("interpreter-body-prepend",  |mut i: Handle| async move {
        let body = i.stack_pop::<List>().await.into_inner();
        let interp = i.stack_pop::<Interp>().await;
        interp.as_ref().body_prepend(body);
        i.stack_push(interp).await;
    });
}

