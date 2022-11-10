
//! OS-level process builtins.
//! Basically closely wraps Rust's API
//! so it can be wrapped even more in pure Worst elsewhere.

use crate::base::*;
use crate::interpreter::{Interpreter, Handle};
use super::util;
use super::io::{or_io_error, port_read_range, port_write_range};
use std::io;
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

#[derive(Clone)]
struct ChildStdin(Rc<RefCell<process::ChildStdin>>);
impl Value for ChildStdin {}
#[derive(Clone)]
struct ChildStdout(Rc<RefCell<process::ChildStdout>>);
impl Value for ChildStdout {}
#[derive(Clone)]
struct ChildStderr(Rc<RefCell<process::ChildStderr>>);
impl Value for ChildStderr {}

impl io::Write for ChildStdin {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.borrow_mut().write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.0.borrow_mut().flush()
    }
}
impl io::Read for ChildStdout {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.borrow_mut().read(buf)
    }
}
impl io::Read for ChildStderr {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.borrow_mut().read(buf)
    }
}

async fn with_command_child(i: &mut Handle,
                            f: impl FnOnce(&mut process::Command,
                                           &mut process::Child)) {
    let mut ch = i.stack_pop::<Child>().await;
    let mut co = i.stack_pop::<Command>().await;
    f(&mut co.as_mut().0.borrow_mut(), &mut ch.as_mut().0.borrow_mut());
    i.stack_push(co).await;
    i.stack_push(ch).await;
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

    // wow
    i.define("process-command-stdin-inherit", |mut i: Handle| async move {
        Command::with(&mut i, |c| { c.stdin(process::Stdio::inherit()); }).await;
    });
    i.define("process-command-stdin-null", |mut i: Handle| async move {
        Command::with(&mut i, |c| { c.stdin(process::Stdio::null()); }).await;
    });
    i.define("process-command-stdin-piped", |mut i: Handle| async move {
        Command::with(&mut i, |c| { c.stdin(process::Stdio::piped()); }).await;
    });
    i.define("process-command-stdout-inherit", |mut i: Handle| async move {
        Command::with(&mut i, |c| { c.stdout(process::Stdio::inherit()); }).await;
    });
    i.define("process-command-stdout-null", |mut i: Handle| async move {
        Command::with(&mut i, |c| { c.stdout(process::Stdio::null()); }).await;
    });
    i.define("process-command-stdout-piped", |mut i: Handle| async move {
        Command::with(&mut i, |c| { c.stdout(process::Stdio::piped()); }).await;
    });
    i.define("process-command-stderr-inherit", |mut i: Handle| async move {
        Command::with(&mut i, |c| { c.stderr(process::Stdio::inherit()); }).await;
    });
    i.define("process-command-stderr-null", |mut i: Handle| async move {
        Command::with(&mut i, |c| { c.stderr(process::Stdio::null()); }).await;
    });
    i.define("process-command-stderr-piped", |mut i: Handle| async move {
        Command::with(&mut i, |c| { c.stderr(process::Stdio::piped()); }).await;
    });

    i.define("process-command-stdin-child-stdout", |mut i: Handle| async move {
        with_command_child(&mut i, |co, ch| {
            co.stdin(ch.stdout.take().expect("child stdout double-take"));
        }).await;
    });
    i.define("process-command-stdin-child-stderr", |mut i: Handle| async move {
        with_command_child(&mut i, |co, ch| {
            co.stdin(ch.stderr.take().expect("child stderr double-take"));
        }).await;
    });
    i.define("process-command-stdout-child-stdin", |mut i: Handle| async move {
        with_command_child(&mut i, |co, ch| {
            co.stdout(ch.stdin.take().expect("child stdin double-take"));
        }).await;
    });
    i.define("process-command-stderr-child-stdin", |mut i: Handle| async move {
        with_command_child(&mut i, |co, ch| {
            co.stderr(ch.stdin.take().expect("child stdin double-take"));
        }).await;
    });

    i.define("process-command-spawn-child", |mut i: Handle| async move {
        let mut c = i.stack_pop::<Command>().await;
        let spawned = c.as_mut().0.borrow_mut().spawn();
        if let Some(child) = or_io_error(&mut i, spawned).await {
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
        if let Some(status) = or_io_error(&mut i, res).await {
            if status.success() {
                i.stack_push(true).await;
            } else if let Some(code) = status.code() {
                i.stack_push(code as i64).await;
            } else {
                i.stack_push(false).await;
            }
        }
    });

    i.define("process-child-stdin-port?", util::type_predicate::<ChildStdin>);
    i.define("process-child-stdout-port?", util::type_predicate::<ChildStdout>);
    i.define("process-child-stderr-port?", util::type_predicate::<ChildStderr>);

    i.define("process-child-stdin-port", |mut i: Handle| async move {
        if let Some(p) = Child::get_mut(&i, |c| c.stdin.take()).await {
            i.stack_push(ChildStdin(Rc::new(RefCell::new(p)))).await;
        } else {
            i.stack_push(false).await;
        }
    });
    i.define("process-child-stdout-port", |mut i: Handle| async move {
        if let Some(p) = Child::get_mut(&i, |c| c.stdout.take()).await {
            i.stack_push(ChildStdout(Rc::new(RefCell::new(p)))).await;
        } else {
            i.stack_push(false).await;
        }
    });
    i.define("process-child-stderr-port", |mut i: Handle| async move {
        if let Some(p) = Child::get_mut(&i, |c| c.stderr.take()).await {
            i.stack_push(ChildStderr(Rc::new(RefCell::new(p)))).await;
        } else {
            i.stack_push(false).await;
        }
    });

    i.define("process-child-stdin-write-range", port_write_range::<ChildStdin>);
    i.define("process-child-stdout-read-range", port_read_range::<ChildStdout>);
    i.define("process-child-stderr-read-range", port_read_range::<ChildStderr>);

}


