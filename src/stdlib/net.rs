
use std::cell::Ref;
use std::rc::Rc;
use std::cell::RefCell;
use std::net;
use std::net::SocketAddr;
use crate::data::*;
use crate::interpreter::exec;
use crate::interpreter::exec::Failure;
use crate::interpreter::Interpreter;
use crate::stdlib::vector::U8Vector;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<UdpSocket>("udp-socket?");
    interpreter.define_type_predicate::<SocketAddr>("socket-addr?");
    interpreter.add_builtin("udp-socket-bind", udp_socket_bind);
    interpreter.add_builtin("udp-socket-recv-from", udp_socket_recv_from);
    interpreter.add_builtin("udp-socket-send-to", udp_socket_send_to);
    interpreter.add_builtin("udp-socket-local-addr", udp_socket_local_addr);

    interpreter.add_builtin("string->socket-addr", string_into_socket_addr);
}

fn udp_socket_bind(interpreter: &mut Interpreter) -> exec::Result<()> {
    let addr = interpreter.stack.pop::<SocketAddr>()?;
    let sock = net::UdpSocket::bind(addr).map(UdpSocket::from);
    interpreter.stack.push_res(sock, source);
    Ok(())
}

fn udp_socket_recv_from(interpreter: &mut Interpreter) -> exec::Result<()> {
    let (mut buf, bufsrc) = interpreter.stack.pop_source::<U8Vector>()?;
    let lenaddr = {
        let sock = interpreter.stack.ref_at::<UdpSocket>(0)?;
        sock.inner().recv_from(&mut buf.inner_mut())
    };

    interpreter.stack.push(Datum::new(buf));
    match lenaddr {
        Ok((len, addr)) => {
            interpreter.stack.push(Datum::new(isize::from_num(len)?));
            interpreter.stack.push(Datum::new(addr));
        },
        Err(e) => {
            interpreter.stack.push(Datum::new(Failure::from(e)));
        },
    }
    Ok(())
}

fn udp_socket_send_to(interpreter: &mut Interpreter) -> exec::Result<()> {
    let addr = interpreter.stack.ref_at::<SocketAddr>(0)?;
    let buf = interpreter.stack.ref_at::<U8Vector>(1)?;
    let sock = interpreter.stack.ref_at::<UdpSocket>(2)?;
    sock.inner().send_to(&buf.inner().as_ref(), addr)?;
    Ok(())
}

fn udp_socket_local_addr(interpreter: &mut Interpreter) -> exec::Result<()> {
    let addr = {
        let sock = interpreter.stack.ref_at::<UdpSocket>(0)?;
        sock.inner().local_addr()?
    };
    interpreter.stack.push(Datum::new(addr));
    Ok(())
}

fn string_into_socket_addr(interpreter: &mut Interpreter) -> exec::Result<()> {
    use std::str::FromStr;
    let a = SocketAddr::from_str(interpreter.stack.pop::<String>()?.as_str());
    interpreter.stack.push_res(a, source);
    Ok(())
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

