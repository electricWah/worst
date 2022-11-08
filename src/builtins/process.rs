
//! OS-level process builtins.
//! Basically closely wraps Rust's API
//! so it can be wrapped even more in pure Worst elsewhere.

use crate::base::*;
use crate::interpreter::{Interpreter, Handle};
use super::util;
use super::io;
use std::process;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct Command(Rc<RefCell<process::Command>>);
impl Value for Command {}

impl Command {
    async fn with(i: &Handle, f: impl FnOnce(&mut process::Command)) {
        let mut c = i.stack_top::<Command>().await;
        f(&mut c.as_mut().0.borrow_mut());
    }
}

#[derive(Clone)]
struct Child(Rc<RefCell<process::Child>>);
impl Value for Child {}

impl Child {
    // async fn with(i: &mut Handle, f: impl FnOnce(&mut process::Command)) {
    //     let mut c = i.stack_pop::<Command>().await;
    //     f(&mut c.as_mut().0.borrow_mut());
    //     i.stack_push(c).await;
    // }
    async fn get<T>(i: &Handle, f: impl FnOnce(&process::Child) -> T) -> T {
        let c = i.stack_top::<Child>().await;
        let r = f(&c.as_ref().0.borrow());
        r
    }
    async fn get_mut<T>(i: &Handle, f: impl FnOnce(&mut process::Child) -> T) -> T {
        let mut c = i.stack_top::<Child>().await;
        let r = f(&mut c.as_mut().0.borrow_mut());
        r
    }
}

/// Install 'em
pub fn install(i: &mut Interpreter) {
    i.define("process-command?", util::type_predicate::<Command>);
    i.define("process-command-create", |mut i: Handle| async move {
        let path = i.stack_pop::<String>().await.into_inner();
        i.stack_push(Command(Rc::new(RefCell::new(process::Command::new(path))))).await;
    });
    i.define("process-command-arg-add", |mut i: Handle| async move {
        let arg = i.stack_pop::<String>().await.into_inner();
        Command::with(&mut i, |c| { c.arg(arg); }).await;
    });
    // i.define("process-command-args", |mut i: Handle| async move {
    //     let args = Command::get(&mut i, |c| {
    //         c.get_args()
    //             .map(OsStr::to_os_string)
    //             .map(OsString::into_string)
    //             // they're all strings because worst put them there
    //             .map(Result::unwrap)
    //             .map(Val::from)
    //             .collect::<Vec<Val>>()
    //     }).await;
    //     i.stack_push(List::from(args)).await;
    // });

    i.define("process-command-env-add", |mut i: Handle| async move {
        let val = i.stack_pop::<String>().await.into_inner();
        let key = i.stack_pop::<String>().await.into_inner();
        Command::with(&mut i, |c| { c.env(key, val); }).await;
    });
    i.define("process-command-env-remove", |mut i: Handle| async move {
        let key = i.stack_pop::<String>().await.into_inner();
        Command::with(&mut i, |c| { c.env_remove(key); }).await;
    });
    i.define("process-command-env-clear", |mut i: Handle| async move {
        Command::with(&mut i, |c| { c.env_clear(); }).await;
    });
    i.define("process-command-directory", |mut i: Handle| async move {
        let dir = i.stack_pop::<String>().await.into_inner();
        Command::with(&mut i, |c| { c.current_dir(dir); }).await;
    });

    i.define("process-command-spawn-child", |mut i: Handle| async move {
        let mut c = i.stack_pop::<Command>().await;
        let spawned = c.as_mut().0.borrow_mut().spawn();
        if let Some(child) = io::or_io_error(&mut i, spawned).await {
            i.stack_push(Child(Rc::new(RefCell::new(child)))).await;
        }
    });

    i.define("process-child?", util::type_predicate::<Child>);
    i.define("process-child-id", |mut i: Handle| async move {
        let id = Child::get(&i, |c| c.id()).await;
        i.stack_push(id as i64).await;
    });
    i.define("process-child-wait", |mut i: Handle| async move {
        let res = Child::get_mut(&i, |c| c.wait()).await;
        if let Some(status) = io::or_io_error(&mut i, res).await {
            if status.success() {
                i.stack_push(true).await;
            } else if let Some(code) = status.code() {
                i.stack_push(code as i64).await;
            } else {
                i.stack_push(false).await;
            }
        }
    });

}


