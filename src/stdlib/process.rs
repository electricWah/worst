
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::process;
use std::rc::Rc;
use data::*;
use parser::*;
use interpreter::Interpreter;
use interpreter::command;
use interpreter::exec;
use stdlib::enumcommand::*;
use stdlib::port::{Port, IsPort};

pub fn install(interpreter: &mut Interpreter) {
    ProcessOp::install(interpreter);
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
            ProcessPort::Stdout => Some(self), // self.proc.0.borrow_mut().stdout.as_mut(),
            ProcessPort::Stderr => Some(self),
        }
    }
    fn as_output(&mut self) -> Option<&mut io::Write> {
        match self.port {
            ProcessPort::Stdin => Some(self), // self.proc.0.borrow_mut().stdin.as_mut(),
            ProcessPort::Stdout => None,
            ProcessPort::Stderr => None,
        }
    }
    fn port_type(&self) -> Option<Type> {
        Some(self.type_of())
    }
}

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ProcessOp {
    MakeCommand,
    CommandAddArgument,
    CommandCd,

    CommandClearEnv,
    CommandSetEnv,

    CommandStdinPipe,
    CommandStdinNull,
    CommandStdinInherit,
    CommandStdoutPipe,
    CommandStdoutNull,
    CommandStdoutInherit,
    CommandStderrPipe,
    CommandStderrNull,
    CommandStderrInherit,

    CommandSpawn,
    IsCommand,

    ProcessHasStdin,
    ProcessHasStdout,
    ProcessHasStderr,
    ProcessStdinPort,
    ProcessStdoutPort,
    ProcessStderrPort,

    ProcessId,
    ProcessKill,

    IsProcessRunning,
    ProcessWait,

    IsProcess,
}

impl EnumCommand for ProcessOp {
    fn as_str(&self) -> &str {
        use self::ProcessOp::*;
        match self {
            MakeCommand => "make-command",
            CommandAddArgument => "command-add-argument",
            CommandCd => "command-cd",
            CommandClearEnv => "command-clear-env",
            CommandSetEnv => "command-set-env",
            CommandStdinPipe => "command-stdin-pipe",
            CommandStdinNull => "command-stdin-null",
            CommandStdinInherit => "command-stdin-inherit",
            CommandStdoutPipe => "command-stdout-pipe",
            CommandStdoutNull => "command-stdout-null",
            CommandStdoutInherit => "command-stdout-inherit",
            CommandStderrPipe => "command-stderr-pipe",
            CommandStderrNull => "command-stderr-null",
            CommandStderrInherit => "command-stderr-inherit",
            CommandSpawn => "command-spawn",
            IsCommand => "command?",
            ProcessHasStdin => "process-has-stdin",
            ProcessHasStdout => "process-has-stdout",
            ProcessHasStderr => "process-has-stderr",
            ProcessStdinPort => "process-stdin-port",
            ProcessStdoutPort => "process-stdout-port",
            ProcessStderrPort => "process-stderr-port",
            ProcessId => "process-id",
            ProcessKill => "process-kill",
            IsProcessRunning => "process-running?",
            ProcessWait => "process-wait",
            IsProcess => "process?",
        }
    }
    fn last() -> Self { ProcessOp::IsProcess }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

impl command::Command for ProcessOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        debug!("ProcessOp: {:?}", self);
        use self::ProcessOp::*;
        match self {
            MakeCommand => {
                let s = interpreter.stack.pop::<String>()?;
                let cmd = Command::new(s);
                interpreter.stack.push(Datum::build().with_source(source).ok(cmd));
            },
            CommandAddArgument => {
                let s = interpreter.stack.pop::<String>()?;
                let mut cmd = interpreter.stack.top_mut::<Command>()?;
                cmd.args.push(s);
            },
            CommandCd => {
                let s = interpreter.stack.pop::<String>()?;
                let mut cmd = interpreter.stack.top_mut::<Command>()?;
                cmd.cd = Some(s);
            },
            CommandClearEnv => {
                let mut cmd = interpreter.stack.top_mut::<Command>()?;
                cmd.env = HashMap::default();
            },
            CommandSetEnv => {
                let v = interpreter.stack.pop::<String>()?;
                let k = interpreter.stack.pop::<String>()?;
                let mut cmd = interpreter.stack.top_mut::<Command>()?;
                cmd.env.insert(k, v);
            },
            CommandStdinPipe => {
                let mut cmd = interpreter.stack.top_mut::<Command>()?;
                cmd.stdin = StdioMode::Pipe;
            },
            CommandStdinNull => {
                let mut cmd = interpreter.stack.top_mut::<Command>()?;
                cmd.stdin = StdioMode::Null;
            },
            CommandStdinInherit => {
                let mut cmd = interpreter.stack.top_mut::<Command>()?;
                cmd.stdin = StdioMode::Inherit;
            },
            CommandStdoutPipe => {
                let mut cmd = interpreter.stack.top_mut::<Command>()?;
                cmd.stdout = StdioMode::Pipe;
            },
            CommandStdoutNull => {
                let mut cmd = interpreter.stack.top_mut::<Command>()?;
                cmd.stdout = StdioMode::Null;
            },
            CommandStdoutInherit => {
                let mut cmd = interpreter.stack.top_mut::<Command>()?;
                cmd.stdout = StdioMode::Inherit;
            },
            CommandStderrPipe => {
                let mut cmd = interpreter.stack.top_mut::<Command>()?;
                cmd.stderr = StdioMode::Pipe;
            },
            CommandStderrNull => {
                let mut cmd = interpreter.stack.top_mut::<Command>()?;
                cmd.stderr = StdioMode::Null;
            },
            CommandStderrInherit => {
                let mut cmd = interpreter.stack.top_mut::<Command>()?;
                cmd.stderr = StdioMode::Inherit;
            },
            CommandSpawn => {
                let proc = {
                    let cmd = interpreter.stack.ref_at::<Command>(0)?;
                    Process::new(cmd.compile().spawn()?)
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(proc));
            },
            IsCommand => {
                let r = interpreter.stack.type_predicate::<Command>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            ProcessHasStdin => {
                let r = {
                    let proc = interpreter.stack.ref_at::<Process>(0)?;
                    proc.0.borrow().stdin.is_some()
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            ProcessHasStdout => {
                let r = {
                    let proc = interpreter.stack.ref_at::<Process>(0)?;
                    proc.0.borrow().stdout.is_some()
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            ProcessHasStderr => {
                let r = {
                    let proc = interpreter.stack.ref_at::<Process>(0)?;
                    proc.0.borrow().stderr.is_some()
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            ProcessStdinPort => {
                let p = interpreter.stack.ref_at::<Process>(0)?.stdin()?;
                interpreter.stack.push(Datum::build().with_source(source).ok(p));
            },
            ProcessStdoutPort => {
                let p = interpreter.stack.ref_at::<Process>(0)?.stdout()?;
                interpreter.stack.push(Datum::build().with_source(source).ok(p));
            },
            ProcessStderrPort => {
                let p = interpreter.stack.ref_at::<Process>(0)?.stderr()?;
                interpreter.stack.push(Datum::build().with_source(source).ok(p));
            },
            ProcessId => {
                let r = {
                    let proc = interpreter.stack.ref_at::<Process>(0)?;
                    proc.0.borrow().id()
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(r)));
            },
            ProcessKill => {
                let mut proc = interpreter.stack.top_mut::<Process>()?;
                if let Err(e) = proc.0.borrow_mut().kill() {
                    if e.kind() != io::ErrorKind::InvalidInput {
                        Err(e)?;
                    }
                }
            },
            IsProcessRunning => {
                let r = {
                    let mut proc = interpreter.stack.top_mut::<Process>()?;
                    proc.0.borrow_mut().try_wait()?.is_none()
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            ProcessWait => {
            },
            IsProcess => {
                let r = interpreter.stack.type_predicate::<Process>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
        }
        Ok(())
    }
}

