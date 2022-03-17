
use std::rc::Rc;
use std::cell::RefCell;

use crate::base::*;
use crate::list::*;
use crate::interpreter::{Builder, Paused, Handle};

#[derive(Debug, Clone)]
enum Interpreter {
    Builder(Builder),
    Paused(Rc<RefCell<Paused>>),
}

impl PartialEq for Interpreter {
    fn eq(&self, other: &Interpreter) -> bool {
        match (self, other) {
            (Interpreter::Builder(a), Interpreter::Builder(b)) => {
                a == b
            },
            (Interpreter::Paused(a), Interpreter::Paused(b)) => {
                Rc::ptr_eq(a, b)
            },
            _ => false,
        }
    }
}
impl Eq for Interpreter {}

impl ImplValue for Interpreter {}

impl Interpreter {
    fn define(&mut self, name: impl Into<String>, def: Val) {
        match self {
            Interpreter::Builder(i) => {
                i.define(name, def);
            },
            Interpreter::Paused(i) => {
                i.borrow_mut().define(name, def);
            },
        }
    }
}

pub fn install(mut i: Builder) -> Builder {
    i.define("interpreter-empty", |mut i: Handle| async move {
        i.stack_push(Interpreter::Builder(Default::default())).await;
    });
    i.define("interpreter-definition-add", |mut i: Handle| async move {
        if let Some(name) = i.stack_pop::<Symbol>().await {
            if let Some(def) = i.stack_pop_val().await {
                if let Some(mut interp) = i.stack_pop::<Interpreter>().await {
                    interp.define(name, def);
                    i.stack_push(interp).await;
                } else {
                    dbg!("stack enfioen");
                }
            } else {
                dbg!("no");
            }
        } else {
            dbg!("no");
        }
    });
    i
}

