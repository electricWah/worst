
use std::io::SeekFrom;
use data::*;
use parser::*;
use interpreter::Interpreter;
use interpreter::command::*;
use interpreter::exec;
use stdlib::enumcommand::*;

use stdlib::vector::data::U8Vector;
use super::data::*;

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum IoOp {
    IsPort,
    IsInputPort,
    IsOutputPort,

    StandardOutputPort,
    StandardInputPort,
    StandardErrorPort,

    // OpenInputString,
    // OpenOutputString,
    // GetOutputString,

    // OpenInputBytevector,
    // OpenOutputBytevector,
    // GetOutputBytevector,

    PortRead,
    PortWrite,

    IsEofObject,
    EofObject,

    // ??? Some kind of pipe-input-to-output
    // given an input port, an output port and a count (of char or u8)

    IsPortUnique,

    IsPortSeekable,
    PortSeekByStart,
    PortSeekByEnd,
    PortSeekByRelative,

    FlushOutputPort,
}

impl EnumCommand for IoOp {
    fn as_str(&self) -> &str {
        use self::IoOp::*;
        match self {
            IsPort => "port?",
            IsInputPort => "input-port?",
            IsOutputPort => "output-port?",
            StandardInputPort => "standard-input-port",
            StandardOutputPort => "standard-output-port",
            StandardErrorPort => "standard-error-port",
            PortRead => "port-read",
            PortWrite => "port-write",
            IsEofObject => "eof-object?",
            EofObject => "eof-object",
            IsPortUnique => "port-unique?",
            IsPortSeekable => "port-seekable?",
            PortSeekByStart => "port-seek/start",
            PortSeekByEnd => "port-seek/end",
            PortSeekByRelative => "port-seek/relative",
            FlushOutputPort => "flush-output-port",
        }
    }
    fn last() -> Self { IoOp::FlushOutputPort }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

impl Command for IoOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::IoOp::*;
        match self {
            &IsPort => {
                let r = interpreter.stack.type_predicate::<Port>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            &IsInputPort => {
                let r = {
                    if interpreter.stack.type_predicate::<Port>(0)? {
                        interpreter.stack.ref_at::<Port>(0)?.is_input()
                    } else {
                        false
                    }
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            &IsOutputPort => {
                let r = {
                    if interpreter.stack.type_predicate::<Port>(0)? {
                        interpreter.stack.ref_at::<Port>(0)?.is_output()
                    } else {
                        false
                    }
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            &StandardInputPort => {
                let p = Port::stdin();
                interpreter.stack.push(Datum::build().with_source(source).ok(p));
            },
            &StandardOutputPort => {
                let p = Port::stdout();
                interpreter.stack.push(Datum::build().with_source(source).ok(p));
            },
            &StandardErrorPort => {
                let p = Port::stderr();
                interpreter.stack.push(Datum::build().with_source(source).ok(p));
            },
            &PortRead => {
                let mut bufd = interpreter.stack.pop_datum()?;
                let c = {
                    let mut buf = bufd.value_mut::<U8Vector>()
                        .map_err(|t| error::WrongType(U8Vector::get_type(), t))?;
                    let mut port = interpreter.stack.top_mut::<Port>()?;
                    port.read_into(buf.inner_mut())?
                };
                interpreter.stack.push(bufd);
                interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(c)));
            },
            &PortWrite => {
                let data = interpreter.stack.pop::<U8Vector>()?;
                let mut port = interpreter.stack.top_mut::<Port>()?;
                port.write(data.into())?;
            },

            &IsPortUnique => {
                let r = {
                    let p = interpreter.stack.ref_at::<Port>(0)?;
                    p.is_unique()
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },

            IsPortSeekable => {
                let r = {
                    let p = interpreter.stack.ref_at::<Port>(0)?;
                    p.can_seek()
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            PortSeekByStart => {
                let seek = {
                    let offs = interpreter.stack.pop::<Number>()?.cast::<u64>()?;
                    let port = interpreter.stack.top_mut::<Port>()?;
                    port.seek(SeekFrom::Start(offs))?
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(seek)));
            },
            PortSeekByEnd => {
                let seek = {
                    let offs = interpreter.stack.pop::<Number>()?.cast::<i64>()?;
                    let port = interpreter.stack.top_mut::<Port>()?;
                    port.seek(SeekFrom::End(offs))?
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(seek)));
            },
            PortSeekByRelative => {
                let seek = {
                    let offs = interpreter.stack.pop::<Number>()?.cast::<i64>()?;
                    let port = interpreter.stack.top_mut::<Port>()?;
                    port.seek(SeekFrom::Current(offs))?
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(seek)));
            },

            &FlushOutputPort => {
                let mut port = interpreter.stack.top_mut::<Port>()?;
                port.flush()?;
            },
            _ => return Err(error::NotImplemented().into()),
        }
        Ok(())
    }
}

