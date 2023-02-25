
//! OS-level process builtins.
//! Basically closely wraps Rust's API
//! so it can be wrapped even more in pure Worst elsewhere.

use crate::base::*;
use crate::interpreter::*;
use super::util::*;
#[cfg(feature = "enable_fs_os")]
use std::fs;
use std::io;
use std::process;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct Command(Rc<RefCell<process::Command>>);
impl Value for Command {}

impl Command {
    fn with(i: &mut Interpreter, f: impl FnOnce(&mut process::Command)) -> BuiltinRet {
        let c = i.stack_top::<Command>()?;
        f(&mut c.as_ref().0.borrow_mut());
        Ok(())
    }
}

#[derive(Clone)]
struct Child(Rc<RefCell<process::Child>>);
impl Value for Child {}

impl Child {
    fn get<T>(i: &mut Interpreter, f: impl FnOnce(&process::Child) -> T) -> BuiltinRet<T> {
        let c = i.stack_top::<Child>()?;
        let r = f(&c.as_ref().0.borrow());
        Ok(r)
    }
    fn get_mut<T>(i: &mut Interpreter, f: impl FnOnce(&mut process::Child) -> T) -> BuiltinRet<T> {
        let c = i.stack_top::<Child>()?;
        let r = f(&mut c.as_ref().0.borrow_mut());
        Ok(r)
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

fn with_command_child(i: &mut Interpreter,
                            f: impl FnOnce(&mut process::Command,
                                           &mut process::Child)) -> BuiltinRet {
    let ch = i.stack_pop::<Child>()?;
    let co = i.stack_pop::<Command>()?;
    f(&mut co.as_ref().0.borrow_mut(), &mut ch.as_ref().0.borrow_mut());
    i.stack_push(co);
    i.stack_push(ch);
    Ok(())
}

#[cfg(feature = "enable_fs_os")]
fn with_command_file_options(i: &mut Interpreter,
                                   f: impl FnOnce(&mut process::Command,
                                                  fs::File)) -> BuiltinRet {
    let opts = i.stack_pop::<fs::OpenOptions>()?;
    let path = i.stack_pop::<String>()?;
    if let Some(file) = or_io_error(i, opts.as_ref().open(path.as_ref())) {
        let co = i.stack_pop::<Command>()?;
        f(&mut co.as_ref().0.borrow_mut(), file);
        i.stack_push(co);
    }
    Ok(())
}

/// Install 'em
pub fn install(i: &mut Interpreter) {
    i.add_builtin("process-command?", type_predicate::<Command>);
    i.add_builtin("process-command-create", |i: &mut Interpreter| {
        let path = i.stack_pop::<String>()?.into_inner();
        i.stack_push(Command(Rc::new(RefCell::new(process::Command::new(path)))));
        Ok(())
    });
    i.add_builtin("process-command-arg-add", |i: &mut Interpreter| {
        let arg = i.stack_pop::<String>()?.into_inner();
        Command::with(i, |c| { c.arg(arg); })
    });

    i.add_builtin("process-command-env-add", |i: &mut Interpreter| {
        let val = i.stack_pop::<String>()?.into_inner();
        let key = i.stack_pop::<String>()?.into_inner();
        Command::with(i, |c| { c.env(key, val); })
    });
    i.add_builtin("process-command-env-remove", |i: &mut Interpreter| {
        let key = i.stack_pop::<String>()?.into_inner();
        Command::with(i, |c| { c.env_remove(key); })
    });
    i.add_builtin("process-command-env-clear", |i: &mut Interpreter| {
        Command::with(i, |c| { c.env_clear(); })
    });
    i.add_builtin("process-command-directory", |i: &mut Interpreter| {
        let dir = i.stack_pop::<String>()?.into_inner();
        Command::with(i, |c| { c.current_dir(dir); })
    });

    // wow
    i.add_builtin("process-command-stdin-inherit", |i: &mut Interpreter| {
        Command::with(i, |c| { c.stdin(process::Stdio::inherit()); })
    });
    i.add_builtin("process-command-stdin-null", |i: &mut Interpreter| {
        Command::with(i, |c| { c.stdin(process::Stdio::null()); })
    });
    i.add_builtin("process-command-stdin-piped", |i: &mut Interpreter| {
        Command::with(i, |c| { c.stdin(process::Stdio::piped()); })
    });
    i.add_builtin("process-command-stdout-inherit", |i: &mut Interpreter| {
        Command::with(i, |c| { c.stdout(process::Stdio::inherit()); })
    });
    i.add_builtin("process-command-stdout-null", |i: &mut Interpreter| {
        Command::with(i, |c| { c.stdout(process::Stdio::null()); })
    });
    i.add_builtin("process-command-stdout-piped", |i: &mut Interpreter| {
        Command::with(i, |c| { c.stdout(process::Stdio::piped()); })
    });
    i.add_builtin("process-command-stderr-inherit", |i: &mut Interpreter| {
        Command::with(i, |c| { c.stderr(process::Stdio::inherit()); })
    });
    i.add_builtin("process-command-stderr-null", |i: &mut Interpreter| {
        Command::with(i, |c| { c.stderr(process::Stdio::null()); })
    });
    i.add_builtin("process-command-stderr-piped", |i: &mut Interpreter| {
        Command::with(i, |c| { c.stderr(process::Stdio::piped()); })
    });

    i.add_builtin("process-command-stdin-child-stdout", |i: &mut Interpreter| {
        with_command_child(i, |co, ch| {
            co.stdin(ch.stdout.take().expect("child stdout double-take"));
        })
    });
    i.add_builtin("process-command-stdin-child-stderr", |i: &mut Interpreter| {
        with_command_child(i, |co, ch| {
            co.stdin(ch.stderr.take().expect("child stderr double-take"));
        })
    });
    i.add_builtin("process-command-stdout-child-stdin", |i: &mut Interpreter| {
        with_command_child(i, |co, ch| {
            co.stdout(ch.stdin.take().expect("child stdin double-take"));
        })
    });
    i.add_builtin("process-command-stderr-child-stdin", |i: &mut Interpreter| {
        with_command_child(i, |co, ch| {
            co.stderr(ch.stdin.take().expect("child stdin double-take"));
        })
    });

    #[cfg(feature = "enable_fs_os")] {
        i.add_builtin("process-command-stdin-file", |i: &mut Interpreter| {
            with_command_file_options(i, |c, f| { c.stdin(f); })
        });
        i.add_builtin("process-command-stdout-file", |i: &mut Interpreter| {
            with_command_file_options(i, |c, f| { c.stdout(f); })
        });
        i.add_builtin("process-command-stderr-file", |i: &mut Interpreter| {
            with_command_file_options(i, |c, f| { c.stderr(f); })
        });
    }

    i.add_builtin("process-command-spawn-child", |i: &mut Interpreter| {
        let c = i.stack_pop::<Command>()?;
        let spawned = c.as_ref().0.borrow_mut().spawn();
        if let Some(child) = or_io_error(i, spawned) {
            i.stack_push(Child(Rc::new(RefCell::new(child))));
        }
        Ok(())
    });

    i.add_builtin("process-child?", type_predicate::<Child>);
    i.add_builtin("process-child-id", |i: &mut Interpreter| {
        let id = Child::get(i, |c| c.id())?;
        i.stack_push(id as i64);
        Ok(())
    });
    i.add_builtin("process-child-wait", |i: &mut Interpreter| {
        let res = Child::get_mut(i, |c| c.wait())?;
        if let Some(status) = or_io_error(i, res) {
            if status.success() {
                i.stack_push(true);
            } else if let Some(code) = status.code() {
                i.stack_push(code as i64);
            } else {
                i.stack_push(false);
            }
        }
        Ok(())
    });

    i.add_builtin("process-child-stdin-port?", type_predicate::<ChildStdin>);
    i.add_builtin("process-child-stdout-port?", type_predicate::<ChildStdout>);
    i.add_builtin("process-child-stderr-port?", type_predicate::<ChildStderr>);

    i.add_builtin("process-child-stdin-port", |i: &mut Interpreter| {
        if let Some(p) = Child::get_mut(i, |c| c.stdin.take())? {
            i.stack_push(ChildStdin(Rc::new(RefCell::new(p))));
        } else {
            i.stack_push(false);
        }
        Ok(())
    });
    i.add_builtin("process-child-stdout-port", |i: &mut Interpreter| {
        if let Some(p) = Child::get_mut(i, |c| c.stdout.take())? {
            i.stack_push(ChildStdout(Rc::new(RefCell::new(p))));
        } else {
            i.stack_push(false);
        }
        Ok(())
    });
    i.add_builtin("process-child-stderr-port", |i: &mut Interpreter| {
        if let Some(p) = Child::get_mut(i, |c| c.stderr.take())? {
            i.stack_push(ChildStderr(Rc::new(RefCell::new(p))));
        } else {
            i.stack_push(false);
        }
        Ok(())
    });

    i.add_builtin("process-child-stdin-write-range", port_write_range::<ChildStdin>);
    i.add_builtin("process-child-stdin-flush", port_flush::<ChildStdin>);
    i.add_builtin("process-child-stdout-read-range", port_read_range::<ChildStdout>);
    i.add_builtin("process-child-stderr-read-range", port_read_range::<ChildStderr>);

}


