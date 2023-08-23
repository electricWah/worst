
//! OS-level process builtins.
//! Basically closely wraps Rust's API
//! so it can be wrapped even more in pure Worst elsewhere.

use crate::base::*;
use crate::interpreter::*;
use super::util;
#[cfg(feature = "enable_fs_os")]
use super::fs::os;
#[cfg(feature = "enable_fs_os")]
use std::fs;
use std::io;
use std::process;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct Command(Rc<RefCell<process::Command>>);
value!(Command);

impl Command {
    fn with(i: &mut Interpreter, f: impl FnOnce(&mut process::Command)) -> BuiltinRet {
        let c = i.stack_top::<Command>()?;
        f(&mut c.as_ref().0.borrow_mut());
        Ok(())
    }
}

#[derive(Clone)]
struct ChildStdin(Rc<RefCell<process::ChildStdin>>);
value!(ChildStdin);
#[derive(Clone)]
struct ChildStdout(Rc<RefCell<process::ChildStdout>>);
value!(ChildStdout);
#[derive(Clone)]
struct ChildStderr(Rc<RefCell<process::ChildStderr>>);
value!(ChildStderr);

enum StdioUnique {
    ChildStdin(process::ChildStdin),
    ChildStdout(process::ChildStdout),
    ChildStderr(process::ChildStderr),
    #[cfg(feature = "enable_fs_os")] File(fs::File),
}

/// [process::Stdio] but clone-able-ish
#[derive(Clone)]
enum Stdio {
    Inherit, Null, Piped,
    Unique(Rc<StdioUnique>),
}
value!(Stdio);

impl From<StdioUnique> for process::Stdio {
    fn from(stdio: StdioUnique) -> Self {
        match stdio {
            StdioUnique::ChildStdin(c) => c.into(),
            StdioUnique::ChildStdout(c) => c.into(),
            StdioUnique::ChildStderr(c) => c.into(),
            #[cfg(feature = "enable_fs_os")] StdioUnique::File(f) => f.into(),
        }
    }
}

impl From<Stdio> for process::Stdio {
    fn from(stdio: Stdio) -> Self {
        match stdio {
            Stdio::Inherit => Self::inherit(),
            Stdio::Null => Self::null(),
            Stdio::Piped => Self::piped(),
            Stdio::Unique(f) =>
                Rc::try_unwrap(f).ok().expect("process-stdio not unique").into(),
        }
    }
}

#[derive(Clone)]
struct Child(Rc<RefCell<process::Child>>);
value!(Child);

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

/// Install 'em
pub fn install(i: &mut Interpreter) {
    util::add_const_type_builtin::<Command>(i, "<process-command>");
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

    i.add_builtin("process-command-stdin", |i: &mut Interpreter| {
        let stdio = i.stack_pop::<Stdio>()?.into_inner();
        Command::with(i, |c| { c.stdin(process::Stdio::from(stdio)); })
    });
    i.add_builtin("process-command-stdout", |i: &mut Interpreter| {
        let stdio = i.stack_pop::<Stdio>()?.into_inner();
        Command::with(i, |c| { c.stdout(process::Stdio::from(stdio)); })
    });
    i.add_builtin("process-command-stderr", |i: &mut Interpreter| {
        let stdio = i.stack_pop::<Stdio>()?.into_inner();
        Command::with(i, |c| { c.stderr(process::Stdio::from(stdio)); })
    });

    i.add_builtin("process-stdio-inherit", |i: &mut Interpreter| {
        i.stack_push(Stdio::Inherit);
        Ok(())
    });
    i.add_builtin("process-stdio-null", |i: &mut Interpreter| {
        i.stack_push(Stdio::Null);
        Ok(())
    });
    i.add_builtin("process-stdio-piped", |i: &mut Interpreter| {
        i.stack_push(Stdio::Piped);
        Ok(())
    });

    i.add_builtin("process-child-stdin->stdio", |i: &mut Interpreter| {
        let ChildStdin(c) = i.stack_pop::<ChildStdin>()?.into_inner();
        match Rc::try_unwrap(c).map(|c| c.into_inner()) {
            Ok(cs) => i.stack_push(Stdio::Unique(Rc::new(StdioUnique::ChildStdin(cs)))),
            Err(_) => i.error("not unique: ChildStdin".to_string())?,
        }
        Ok(())
    });
    i.add_builtin("process-child-stdout->stdio", |i: &mut Interpreter| {
        let ChildStdout(c) = i.stack_pop::<ChildStdout>()?.into_inner();
        match Rc::try_unwrap(c).map(|c| c.into_inner()) {
            Ok(cs) => i.stack_push(Stdio::Unique(Rc::new(StdioUnique::ChildStdout(cs)))),
            Err(_) => i.error("not unique: ChildStdout".to_string())?,
        }
        Ok(())
    });
    i.add_builtin("process-child-stderr->stdio", |i: &mut Interpreter| {
        let ChildStderr(c) = i.stack_pop::<ChildStderr>()?.into_inner();
        match Rc::try_unwrap(c).map(|c| c.into_inner()) {
            Ok(cs) => i.stack_push(Stdio::Unique(Rc::new(StdioUnique::ChildStderr(cs)))),
            Err(_) => i.error("not unique: ChildStderr".to_string())?,
        }
        Ok(())
    });

    #[cfg(feature = "enable_fs_os")]
    i.add_builtin("file->process-stdio", |i: &mut Interpreter| {
        let f = i.stack_pop::<os::File>()?.into_inner();
        match f.try_into_inner() {
            Ok(f) => i.stack_push(Stdio::Unique(Rc::new(StdioUnique::File(f)))),
            Err(_) => i.error("not unique: File".to_string())?,
        }
        Ok(())
    });

    i.add_builtin("process-command-spawn-child", |i: &mut Interpreter| {
        let c = i.stack_pop::<Command>()?;
        let spawned = c.as_ref().0.borrow_mut().spawn();
        if let Some(child) = util::or_io_error(i, spawned) {
            i.stack_push(Child(Rc::new(RefCell::new(child))));
        }
        Ok(())
    });

    util::add_const_type_builtin::<Child>(i, "<process-child>");
    i.add_builtin("process-child-id", |i: &mut Interpreter| {
        let id = Child::get(i, |c| c.id())?;
        i.stack_push(id as i64);
        Ok(())
    });
    i.add_builtin("process-child-wait", |i: &mut Interpreter| {
        let res = Child::get_mut(i, |c| c.wait())?;
        if let Some(status) = util::or_io_error(i, res) {
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

    util::add_const_type_builtin::<ChildStdin>(i, "<process-child-stdin-port>");
    util::add_const_type_builtin::<ChildStdout>(i, "<process-child-stdout-port>");
    util::add_const_type_builtin::<ChildStderr>(i, "<process-child-stderr-port>");

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

    i.add_builtin("process-child-stdin-write-range", util::port_write_range::<ChildStdin>);
    i.add_builtin("process-child-stdin-flush", util::port_flush::<ChildStdin>);
    i.add_builtin("process-child-stdout-read-range", util::port_read_range::<ChildStdout>);
    i.add_builtin("process-child-stderr-read-range", util::port_read_range::<ChildStderr>);

}


