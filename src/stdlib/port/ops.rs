
use std::io::SeekFrom;
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;

use crate::stdlib::vector::data::U8Vector;
use super::data::*;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<Port>("port?");
    // interpreter.define_type_predicate::<EofObject>("eof-object?");

    interpreter.add_builtin("input-port?", is_input_port);
    interpreter.add_builtin("output-port?", is_output_port);
    interpreter.add_builtin("standard-input-port", standard_input_port);
    interpreter.add_builtin("standard-output-port", standard_output_port);
    interpreter.add_builtin("standard-error-port", standard_error_port);
    interpreter.add_builtin("port-read", port_read);
    interpreter.add_builtin("port-write", port_write);
    // interpreter.add_builtin("eof-object", eof_object);
    interpreter.add_builtin("port-unique?", is_port_unique);
    interpreter.add_builtin("port-seekable?", is_port_seekable);
    interpreter.add_builtin("port-seek/start", port_seek_start);
    interpreter.add_builtin("port-seek/end", port_seek_end);
    interpreter.add_builtin("port-seek/relative", port_seek_relative);
    interpreter.add_builtin("output-port-flush", output_port_flush);
}

fn is_input_port(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = {
        if interpreter.stack.type_predicate::<Port>(0)? {
            interpreter.stack.ref_at::<Port>(0)?.is_input()
        } else {
            false
        }
    };
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(r));
    Ok(())
}

fn is_output_port(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = {
        if interpreter.stack.type_predicate::<Port>(0)? {
            interpreter.stack.ref_at::<Port>(0)?.is_output()
        } else {
            false
        }
    };
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(r));
    Ok(())
}

fn standard_input_port(interpreter: &mut Interpreter) -> exec::Result<()> {
    let p = Port::stdin();
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(p));
    Ok(())
}

fn standard_output_port(interpreter: &mut Interpreter) -> exec::Result<()> {
    let p = Port::stdout();
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(p));
    Ok(())
}

fn standard_error_port(interpreter: &mut Interpreter) -> exec::Result<()> {
    let p = Port::stderr();
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(p));
    Ok(())
}

fn port_read(interpreter: &mut Interpreter) -> exec::Result<()> {
    let mut bufd = interpreter.stack.pop_datum()?;
    let c = {
        let buf = bufd.value_mut::<U8Vector>()
            .map_err(|t| error::WrongType(U8Vector::get_type(), t))?;
        let port = interpreter.stack.top_mut::<Port>()?;
        port.read_into(buf.inner_mut())?
    };
    interpreter.stack.push(bufd);
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(isize::from_num(c)?));
    Ok(())
}

fn port_write(interpreter: &mut Interpreter) -> exec::Result<()> {
    let data = interpreter.stack.pop::<U8Vector>()?;
    let port = interpreter.stack.top_mut::<Port>()?;
    port.write(data.into())?;
    Ok(())
}

// fn eof_object(_interpreter: &mut Interpreter) -> exec::Result<()> {
//     Err(error::NotImplemented().into())
// }

fn is_port_unique(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = {
        let p = interpreter.stack.ref_at::<Port>(0)?;
        p.is_unique()
    };
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(r));
    Ok(())
}

fn is_port_seekable(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = {
        let p = interpreter.stack.ref_at::<Port>(0)?;
        p.can_seek()
    };
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(r));
    Ok(())
}

fn port_seek_start(interpreter: &mut Interpreter) -> exec::Result<()> {
    let seek = {
        let offs = interpreter.stack.pop::<isize>()?.cast::<u64>()?;
        let port = interpreter.stack.top_mut::<Port>()?;
        port.seek(SeekFrom::Start(offs))?
    };
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(isize::from_num(seek)?));
    Ok(())
}

fn port_seek_end(interpreter: &mut Interpreter) -> exec::Result<()> {
    let seek = {
        let offs = interpreter.stack.pop::<isize>()?.cast::<i64>()?;
        let port = interpreter.stack.top_mut::<Port>()?;
        port.seek(SeekFrom::End(offs))?
    };
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(isize::from_num(seek)?));
    Ok(())
}

fn port_seek_relative(interpreter: &mut Interpreter) -> exec::Result<()> {
    let seek = {
        let offs = interpreter.stack.pop::<isize>()?.cast::<i64>()?;
        let port = interpreter.stack.top_mut::<Port>()?;
        port.seek(SeekFrom::Current(offs))?
    };
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(isize::from_num(seek)?));
    Ok(())
}

fn output_port_flush(interpreter: &mut Interpreter) -> exec::Result<()> {
    let port = interpreter.stack.top_mut::<Port>()?;
    port.flush()?;
    Ok(())
}

