
use std::net::SocketAddr;
use data::*;
use parser::Source;
use interpreter::command::*;
use interpreter::exec;
use interpreter::exec::Failure;
use interpreter::Interpreter;
use stdlib::enumcommand::*;

pub fn install(interpreter: &mut Interpreter) {
    NetOp::install(interpreter);
}

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum NetOp {
    StringToSocketAddr,
    IsSocketAddr,
}

impl EnumCommand for NetOp {
    fn as_str(&self) -> &str {
        use self::NetOp::*;
        match self {
            StringToSocketAddr => "string->socket-addr",
            IsSocketAddr => "socket-addr?",
        }
    }
    fn last() -> Self { NetOp::IsSocketAddr }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

impl Command for NetOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        debug!("NetOp: {:?}", self);
        use self::NetOp::*;
        match self {
            StringToSocketAddr => {
                use std::str::FromStr;
                let a = SocketAddr::from_str(interpreter.stack.pop::<String>()?.as_str());
                match a {
                    Ok(addr) => {
                        interpreter.stack.push(Datum::build().with_source(source).ok(addr));
                    },
                    Err(e) => {
                        interpreter.stack.push(Datum::new(Failure::from(e)));
                    },
                }
            },
            IsSocketAddr => {
                let r = interpreter.stack.type_predicate::<SocketAddr>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
        }
        Ok(())
    }
}

impl StaticType for SocketAddr {
    fn static_type() -> Type {
        Type::new("socket-addr")
    }
}

impl DefaultValueClone for SocketAddr {}
impl ValueDebugDescribe for SocketAddr {}
impl ValueShow for SocketAddr {}
impl ValueEq for SocketAddr {}
impl ValueHash for SocketAddr {}
impl Value for SocketAddr {}

