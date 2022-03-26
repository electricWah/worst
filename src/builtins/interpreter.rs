
use std::rc::Rc;
use std::cell::RefCell;

use crate::base::*;
use crate::interpreter::{Builder, Paused, Handle};

#[derive(Debug, Clone)]
struct Interpreter(Rc<RefCell<Paused>>);

impl PartialEq for Interpreter {
    fn eq(&self, other: &Interpreter) -> bool { Rc::ptr_eq(&self.0, &other.0) }
}
impl Eq for Interpreter {}
impl ImplValue for Interpreter {}

impl Interpreter {
    fn define(&mut self, name: impl Into<String>, def: Val) {
        self.0.borrow_mut().define(name, def);
    }
    fn run(&mut self) -> bool {
        self.0.borrow_mut().run()
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter(Rc::new(RefCell::new(Builder::default().eval(|_: Handle| async move {}))))
    }
}

pub fn install(mut i: Builder) -> Builder {
    i.define("interpreter-empty", |mut i: Handle| async move {
        i.stack_push(Interpreter::default()).await;
    });
    i.define("interpreter-run",  |mut i: Handle| async move {
        let mut interp = i.stack_pop::<Interpreter>().await;
        let r = interp.run();
        i.stack_push(interp).await;
        i.stack_push(r).await;
    });
    i.define("interpreter-stack-push",  |mut i: Handle| async move {
        let v = i.stack_pop_val().await;
        let mut interp = i.stack_pop::<Interpreter>().await;
        interp.0.borrow_mut().stack_push(v);
        i.stack_push(interp).await;
    });
    i.define("interpreter-stack-get",  |mut i: Handle| async move {
        let mut interp = i.stack_pop::<Interpreter>().await;
        let s = interp.0.borrow_mut().stack_ref().clone();
        i.stack_push(interp).await;
        i.stack_push(s).await;
    });
    i.define("interpreter-definition-add", |mut i: Handle| async move {
        let name = i.stack_pop::<Symbol>().await;
        let def = i.stack_pop_val().await;
        i.with_stack_top_mut(move |interp: &mut Interpreter| interp.define(name, def)).await;
    });
    i
}

