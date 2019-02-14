
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::process;
use std::rc::Rc;
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;
use crate::stdlib::port::{Port, IsPort};

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<Command>("command?");
    interpreter.define_type_predicate::<Process>("process?");
    interpreter.add_builtin("make-command", make_command);
    interpreter.add_builtin("command-add-argument", command_add_argument);
    interpreter.add_builtin("command-cd", command_cd);
    interpreter.add_builtin("command-clear-env", command_clear_env);
    interpreter.add_builtin("command-set-env", command_set_env);
    interpreter.add_builtin("command-stdin-pipe", command_stdin_pipe);
    interpreter.add_builtin("command-stdin-null", command_stdin_null);
    interpreter.add_builtin("command-stdin-inherit", command_stdin_inherit);
    interpreter.add_builtin("command-stdout-pipe", command_stdout_pipe);
    interpreter.add_builtin("command-stdout-null", command_stdout_null);
    interpreter.add_builtin("command-stdout-inherit", command_stdout_inherit);
    interpreter.add_builtin("command-stderr-pipe", command_stderr_pipe);
    interpreter.add_builtin("command-stderr-null", command_stderr_null);
    interpreter.add_builtin("command-stderr-inherit", command_stderr_inherit);
    interpreter.add_builtin("command-spawn", command_spawn);
    interpreter.add_builtin("process-has-stdin", process_has_stdin);
    interpreter.add_builtin("process-has-stdout", process_has_stdout);
    interpreter.add_builtin("process-has-stderr", process_has_stderr);
    interpreter.add_builtin("process-stdin-port", process_stdin_port);
    interpreter.add_builtin("process-stdout-port", process_stdout_port);
    interpreter.add_builtin("process-stderr-port", process_stderr_port);
    interpreter.add_builtin("process-id", process_id);
    interpreter.add_builtin("process-kill", process_kill);
    interpreter.add_builtin("process-running?", is_process_running);
    interpreter.add_builtin("process-wait", process_wait);
}

fn make_command(interpreter: &mut Interpreter) -> exec::Result<()> {
    let s = interpreter.stack.pop::<String>()?;
    let cmd = Command::new(s);
    interpreter.stack.push(Datum::new(cmd));
    Ok(())
}

fn command_add_argument(interpreter: &mut Interpreter) -> exec::Result<()> {
    let s = interpreter.stack.pop::<String>()?;
    let cmd = interpreter.stack.top_mut::<Command>()?;
    cmd.args.push(s);
    Ok(())
}

fn command_cd(interpreter: &mut Interpreter) -> exec::Result<()> {
    let s = interpreter.stack.pop::<String>()?;
    let cmd = interpreter.stack.top_mut::<Command>()?;
    cmd.cd = Some(s);
    Ok(())
}

fn command_clear_env(interpreter: &mut Interpreter) -> exec::Result<()> {
    let cmd = interpreter.stack.top_mut::<Command>()?;
    cmd.env = HashMap::default();
    Ok(())
}

fn command_set_env(interpreter: &mut Interpreter) -> exec::Result<()> {
    let v = interpreter.stack.pop::<String>()?;
    let k = interpreter.stack.pop::<String>()?;
    let cmd = interpreter.stack.top_mut::<Command>()?;
    cmd.env.insert(k, v);
    Ok(())
}

fn command_stdin_pipe(interpreter: &mut Interpreter) -> exec::Result<()> {
    let cmd = interpreter.stack.top_mut::<Command>()?;
    cmd.stdin = StdioMode::Pipe;
    Ok(())
}

fn command_stdin_null(interpreter: &mut Interpreter) -> exec::Result<()> {
    let cmd = interpreter.stack.top_mut::<Command>()?;
    cmd.stdin = StdioMode::Null;
    Ok(())
}

fn command_stdin_inherit(interpreter: &mut Interpreter) -> exec::Result<()> {
    let cmd = interpreter.stack.top_mut::<Command>()?;
    cmd.stdin = StdioMode::Inherit;
    Ok(())
}

fn command_stdout_pipe(interpreter: &mut Interpreter) -> exec::Result<()> {
    let cmd = interpreter.stack.top_mut::<Command>()?;
    cmd.stdout = StdioMode::Pipe;
    Ok(())
}

fn command_stdout_null(interpreter: &mut Interpreter) -> exec::Result<()> {
    let cmd = interpreter.stack.top_mut::<Command>()?;
    cmd.stdout = StdioMode::Null;
    Ok(())
}

fn command_stdout_inherit(interpreter: &mut Interpreter) -> exec::Result<()> {
    let cmd = interpreter.stack.top_mut::<Command>()?;
    cmd.stdout = StdioMode::Inherit;
    Ok(())
}

fn command_stderr_pipe(interpreter: &mut Interpreter) -> exec::Result<()> {
    let cmd = interpreter.stack.top_mut::<Command>()?;
    cmd.stderr = StdioMode::Pipe;
    Ok(())
}

fn command_stderr_null(interpreter: &mut Interpreter) -> exec::Result<()> {
    let cmd = interpreter.stack.top_mut::<Command>()?;
    cmd.stderr = StdioMode::Null;
    Ok(())
}

fn command_stderr_inherit(interpreter: &mut Interpreter) -> exec::Result<()> {
    let cmd = interpreter.stack.top_mut::<Command>()?;
    cmd.stderr = StdioMode::Inherit;
    Ok(())
}

fn command_spawn(interpreter: &mut Interpreter) -> exec::Result<()> {
    let proc = {
        let cmd = interpreter.stack.ref_at::<Command>(0)?;
        Process::new(cmd.compile().spawn()?)
    };
    interpreter.stack.push(Datum::new(proc));
    Ok(())
}

fn process_has_stdin(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = {
        let proc = interpreter.stack.ref_at::<Process>(0)?;
        proc.0.borrow().stdin.is_some()
    };
    interpreter.stack.push(Datum::new(r));
    Ok(())
}

fn process_has_stdout(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = {
        let proc = interpreter.stack.ref_at::<Process>(0)?;
        proc.0.borrow().stdout.is_some()
    };
    interpreter.stack.push(Datum::new(r));
    Ok(())
}

fn process_has_stderr(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = {
        let proc = interpreter.stack.ref_at::<Process>(0)?;
        proc.0.borrow().stderr.is_some()
    };
    interpreter.stack.push(Datum::new(r));
    Ok(())
}

fn process_stdin_port(interpreter: &mut Interpreter) -> exec::Result<()> {
    let p = interpreter.stack.ref_at::<Process>(0)?.stdin()?;
    interpreter.stack.push(Datum::new(p));
    Ok(())
}

fn process_stdout_port(interpreter: &mut Interpreter) -> exec::Result<()> {
    let p = interpreter.stack.ref_at::<Process>(0)?.stdout()?;
    interpreter.stack.push(Datum::new(p));
    Ok(())
}

fn process_stderr_port(interpreter: &mut Interpreter) -> exec::Result<()> {
    let p = interpreter.stack.ref_at::<Process>(0)?.stderr()?;
    interpreter.stack.push(Datum::new(p));
    Ok(())
}

fn process_id(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = {
        let proc = interpreter.stack.ref_at::<Process>(0)?;
        proc.0.borrow().id()
    };
    interpreter.stack.push(Datum::new(isize::from_num(r)?));
    Ok(())
}

fn process_kill(interpreter: &mut Interpreter) -> exec::Result<()> {
    let proc = interpreter.stack.top_mut::<Process>()?;
    if let Err(e) = proc.0.borrow_mut().kill() {
        if e.kind() != io::ErrorKind::InvalidInput {
            Err(e)?;
        }
    }
    Ok(())
}

fn is_process_running(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = {
        let proc = interpreter.stack.top_mut::<Process>()?;
        proc.0.borrow_mut().try_wait()?.is_none()
    };
    interpreter.stack.push(Datum::new(r));
    Ok(())
}

fn process_wait(interpreter: &mut Interpreter) -> exec::Result<()> {
    let proc = interpreter.stack.top_mut::<Process>()?;
    proc.0.borrow_mut().wait()?;
    Ok(())
}


#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum StdioMode { Inherit, Pipe, Null }
impl Into<process::Stdio> for StdioMode {
    fn into(self) -> process::Stdio {
        use self::StdioMode::*;
        match self {
            Inherit => process::Stdio::inherit(),
            Pipe => process::Stdio::piped(),
            Null => process::Stdio::null(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Command {
    program: String,
    args: Vec<String>,
    cd: Option<String>,
    clear_env: bool,
    env: HashMap<String, String>,
    stdin: StdioMode,
    stdout: StdioMode,
    stderr: StdioMode,
}
impl StaticType for Command {
    fn static_type() -> Type {
        Type::new("command")
    }
}
impl DefaultValueEq for Command {}
impl DefaultValueClone for Command {}
impl ValueHash for Command {}
impl ValueDebugDescribe for Command {}
impl ValueShow for Command {}
impl Value for Command {}

impl Command {
    fn new(program: String) -> Self {
        Command {
            program,
            args: vec![],
            cd: None,
            clear_env: false,
            env: HashMap::default(),
            stdin: StdioMode::Inherit,
            stdout: StdioMode::Inherit,
            stderr: StdioMode::Inherit,
        }
    }
    fn compile(&self) -> process::Command {
        let mut proc = process::Command::new(&self.program);
        proc.args(&self.args)
            .stdin(self.stdin)
            .stdout(self.stdout)
            .stderr(self.stderr);
        if let Some(cd) = &self.cd {
            proc.current_dir(&cd);
        }
        if self.clear_env {
            proc.env_clear();
        }
        proc.envs(&self.env);
        proc
    }
}

#[derive(Debug, Clone)]
struct Process(Rc<RefCell<process::Child>>);
impl StaticType for Process {
    fn static_type() -> Type {
        Type::new("process")
    }
}

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for Process {}

impl DefaultValueEq for Process {}
impl DefaultValueClone for Process {}
impl ValueHash for Process {}
impl ValueDebugDescribe for Process {}
impl ValueShow for Process {}
impl Value for Process {}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct NoSuchPort();
impl error::Error for NoSuchPort {}

impl fmt::Display for NoSuchPort {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Process does not have that port")
    }
}

impl Process {
    pub fn new(child: process::Child) -> Self {
        Process(Rc::new(RefCell::new(child)))
    }
    pub fn stdin(&self) -> exec::Result<Port> {
        if self.0.borrow().stdin.is_none() {
            Err(NoSuchPort())?;
        }
        Ok(Port::new(ProcessPortHandle::stdin(self.clone())))
    }
    pub fn stdout(&self) -> exec::Result<Port> {
        if self.0.borrow().stdout.is_none() {
            Err(NoSuchPort())?;
        }
        Ok(Port::new(ProcessPortHandle::stdout(self.clone())))
    }
    pub fn stderr(&self) -> exec::Result<Port> {
        if self.0.borrow().stderr.is_none() {
            Err(NoSuchPort())?;
        }
        Ok(Port::new(ProcessPortHandle::stderr(self.clone())))
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum ProcessPort { Stdin, Stdout, Stderr }

#[derive(Eq, PartialEq, Debug, Clone)]
struct ProcessPortHandle {
    proc: Process,
    port: ProcessPort,
}

impl ProcessPortHandle {
    fn stdin(proc: Process) -> Self {
        ProcessPortHandle { proc, port: ProcessPort::Stdin }
    }
    fn stdout(proc: Process) -> Self {
        ProcessPortHandle { proc, port: ProcessPort::Stdout }
    }
    fn stderr(proc: Process) -> Self {
        ProcessPortHandle { proc, port: ProcessPort::Stderr }
    }
}

impl io::Write for ProcessPortHandle {
    fn write(&mut self, w: &[u8]) -> io::Result<usize> {
        let mut p = self.proc.0.borrow_mut();
        match self.port {
            ProcessPort::Stdin =>
                if let Some(ref mut port) = &mut p.stdin {
                    return port.write(w);
                },
            ProcessPort::Stdout => {},
            ProcessPort::Stderr => {},
        }
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Not writable"))
    }
    fn flush(&mut self) -> io::Result<()> {
        let mut p = self.proc.0.borrow_mut();
        match self.port {
            ProcessPort::Stdin =>
                if let Some(ref mut port) = &mut p.stdin {
                    return port.flush();
                },
            ProcessPort::Stdout => {},
            ProcessPort::Stderr => {},
        }
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Not writable"))
    }
}

impl io::Read for ProcessPortHandle {
    fn read(&mut self, r: &mut [u8]) -> io::Result<usize> {
        let mut p = self.proc.0.borrow_mut();
        match self.port {
            ProcessPort::Stdin => {},
            ProcessPort::Stdout =>
                if let Some(ref mut port) = &mut p.stdout {
                    return port.read(r);
                },
            ProcessPort::Stderr =>
                if let Some(ref mut port) = &mut p.stderr {
                    return port.read(r);
                },
        }
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Not readable"))
    }
}

impl HasType for ProcessPortHandle {
    fn type_of(&self) -> Type {
        let t = match self.port {
            ProcessPort::Stdin => "process-stdin-port",
            ProcessPort::Stdout => "process-stdout-port",
            ProcessPort::Stderr => "process-stderr-port",
        };
        Type::new(t)
    }
}

impl IsPort for ProcessPortHandle {
    fn is_input(&self) -> bool { self.port != ProcessPort::Stdin }
    fn is_output(&self) -> bool { self.port == ProcessPort::Stdin }
    fn as_input(&mut self) -> Option<&mut io::Read> {
        match self.port {
            ProcessPort::Stdin => None,
            ProcessPort::Stdout => Some(self),
            ProcessPort::Stderr => Some(self),
        }
    }
    fn as_output(&mut self) -> Option<&mut io::Write> {
        match self.port {
            ProcessPort::Stdin => Some(self),
            ProcessPort::Stdout => None,
            ProcessPort::Stderr => None,
        }
    }
    fn port_type(&self) -> Option<Type> {
        Some(self.type_of())
    }
}

