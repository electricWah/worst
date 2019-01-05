
use std::cell::Ref;
use std::rc::Rc;
use std::cell::RefCell;
use std::net;
use std::net::SocketAddr;
use crate::data::*;
use crate::parser::Source;
use crate::interpreter::command::*;
use crate::interpreter::exec;
use crate::interpreter::exec::Failure;
use crate::interpreter::Interpreter;
use crate::stdlib::enumcommand::*;
use crate::stdlib::vector::U8Vector;

pub fn install(interpreter: &mut Interpreter) {
    NetOp::install(interpreter);
}

#[derive(Debug, Clone)]
struct UdpSocket(Rc<RefCell<net::UdpSocket>>);

impl UdpSocket {
    pub fn inner(&self) -> Ref<net::UdpSocket> {
        self.0.borrow()
    }
}

impl From<net::UdpSocket> for UdpSocket {
    fn from(the: net::UdpSocket) -> Self {
        UdpSocket(Rc::new(RefCell::new(the)))
    }
}

impl StaticType for UdpSocket {
    fn static_type() -> Type {
        Type::new("udp-socket")
    }
}

impl DefaultValueClone for UdpSocket {}
impl ValueDebugDescribe for UdpSocket {}
impl ValueShow for UdpSocket {}
impl ValueEq for UdpSocket {}
impl ValueHash for UdpSocket {}
impl Value for UdpSocket {}

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum NetOp {
    UdpSocketBind,
    UdpSocketRecvFrom,
    // UdpSocketPeekFrom,
    UdpSocketSendTo,
    UdpSocketLocalAddr,
    IsUdpSocket,

    StringToSocketAddr,
    IsSocketAddr,
}

impl EnumCommand for NetOp {
    fn as_str(&self) -> &str {
        use self::NetOp::*;
        match self {
            UdpSocketBind => "udp-socket-bind",
            UdpSocketRecvFrom => "udp-socket-recv-from",
            // UdpSocketPeekFrom => "udp-socket-peek-from",
            UdpSocketSendTo => "udp-socket-send-to",
            UdpSocketLocalAddr => "udp-socket-local-addr",
            IsUdpSocket => "udp-socket?",

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
            UdpSocketBind => {
                let addr = interpreter.stack.pop::<SocketAddr>()?;
                let sock = net::UdpSocket::bind(addr).map(UdpSocket::from);
                interpreter.stack.push_res(sock, source);
            },
            UdpSocketRecvFrom => {
                let (mut buf, bufsrc) = interpreter.stack.pop_source::<U8Vector>()?;
                let lenaddr = {
                    let sock = interpreter.stack.ref_at::<UdpSocket>(0)?;
                    sock.inner().recv_from(&mut buf.inner_mut())
                };

                interpreter.stack.push(Datum::build().with_source(bufsrc).ok(buf));
                match lenaddr {
                    Ok((len, addr)) => {
                        interpreter.stack.push(Datum::new(Number::exact(len)));
                        interpreter.stack.push(Datum::new(addr));
                    },
                    Err(e) => {
                        interpreter.stack.push(Datum::new(Failure::from(e)));
                    },
                }
            },
            // UdpSocketPeekFrom => {
            // },
            UdpSocketSendTo => {
                let addr = interpreter.stack.ref_at::<SocketAddr>(0)?;
                let buf = interpreter.stack.ref_at::<U8Vector>(1)?;
                let sock = interpreter.stack.ref_at::<UdpSocket>(2)?;
                sock.inner().send_to(&buf.inner().as_ref(), addr)?;
            },
            UdpSocketLocalAddr => {
                let addr = {
                    let sock = interpreter.stack.ref_at::<UdpSocket>(0)?;
                    sock.inner().local_addr()?
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(addr));
            },
            IsUdpSocket => {
                let r = interpreter.stack.type_predicate::<UdpSocket>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            StringToSocketAddr => {
                use std::str::FromStr;
                let a = SocketAddr::from_str(interpreter.stack.pop::<String>()?.as_str());
                interpreter.stack.push_res(a, source);
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

