
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
    fn define(&mut self, name: impl Into<String>, def: Val) {
        self.0.borrow_mut().define(name, def);
    }
    fn call(&mut self, name: impl Into<Symbol>) {
        self.0.borrow_mut().call(name);
    }
    fn run(&mut self) -> Option<Val> {
        self.0.borrow_mut().run()
    }
}

impl Default for Interp {
    fn default() -> Self {
        Interp(Rc::new(RefCell::new(Interpreter::default())))
    }
}

pub fn install(i: &mut Interpreter) {
    i.define("interpreter-empty", |mut i: Handle| async move {
        i.stack_push(Interp::default()).await;
    });
    i.define("interpreter-run",  |mut i: Handle| async move {
        let mut interp = i.stack_pop::<Interp>().await;
        let r = interp.run();
        i.stack_push(interp).await;
        match r {
            None => i.stack_push(true).await,
            Some(e) => {
                i.stack_push(e).await;
                i.stack_push(false).await;
            },
        }
    });
    i.define("interpreter-reset",  |mut i: Handle| async move {
        let interp = i.stack_pop::<Interp>().await;
        interp.0.borrow_mut().reset();
        i.stack_push(interp).await;
    });
    i.define("interpreter-stack-length",  |mut i: Handle| async move {
        let interp = i.stack_pop::<Interp>().await;
        let len = interp.0.borrow_mut().stack_len();
        i.stack_push(interp).await;
        i.stack_push(len as i32).await;
    });
    i.define("interpreter-stack-push",  |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        let interp = i.stack_pop::<Interp>().await;
        interp.0.borrow_mut().stack_push(v);
        i.stack_push(interp).await;
    });
    i.define("interpreter-stack-pop",  |mut i: Handle| async move {
        let interp = i.stack_pop::<Interp>().await;
        let v = interp.0.borrow_mut().stack_pop_val().unwrap_or_else(|| false.into());
        i.stack_push(interp).await;
        i.stack_push(v).await;
    });
    i.define("interpreter-stack-get",  |mut i: Handle| async move {
        let interp = i.stack_pop::<Interp>().await;
        let s = interp.0.borrow_mut().stack_ref().clone();
        i.stack_push(interp).await;
        i.stack_push(s).await;
    });
    i.define("interpreter-definition-add", |mut i: Handle| async move {
        let name = i.stack_pop::<Symbol>().await;
        let def = i.stack_pop_val().await;
        let mut interp = i.stack_pop::<Interp>().await;
        interp.define(name, def);
        i.stack_push(interp).await;
    });
    i.define("interpreter-definition-remove", |mut i: Handle| async move {
        let name = i.stack_pop::<Symbol>().await;
        let interp = i.stack_pop::<Interp>().await;
        interp.0.borrow_mut().definition_remove(name);
        i.stack_push(interp).await;
    });
    i.define("interpreter-call", |mut i: Handle| async move {
        let name = i.stack_pop::<Symbol>().await;
        let mut interp = i.stack_pop::<Interp>().await;
        interp.call(name);
        i.stack_push(interp).await;
    });
    i.define("interpreter-body-push",  |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        let interp = i.stack_pop::<Interp>().await;
        interp.0.borrow_mut().body_mut().push(v);
        i.stack_push(interp).await;
    });
    i.define("interpreter-body-prepend",  |mut i: Handle| async move {
        let body = i.stack_pop::<List>().await;
        let interp = i.stack_pop::<Interp>().await;
        interp.0.borrow_mut().body_mut().prepend(body);
        i.stack_push(interp).await;
    });
}

