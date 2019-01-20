
use std::fmt;
use std::str;
use crate::data::*;
use crate::data::error::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;

use crate::stdlib::vector::data::*;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<String>("string?");
    interpreter.add_builtin("string->list", string_into_list);
    interpreter.add_builtin("string-length", string_length);
    interpreter.add_builtin("string-get", string_get);
    interpreter.add_builtin("string-set", string_set);
    interpreter.add_builtin("string-compare", string_compare);
    interpreter.add_builtin("string-compare/ci", string_compare_ci);
    interpreter.add_builtin("string-upcase", string_upcase);
    interpreter.add_builtin("string-downcase", string_downcase);
    interpreter.add_builtin("string-append", string_append);
    interpreter.add_builtin("string-push", string_push);
    interpreter.add_builtin("string-pop", string_pop);
    interpreter.add_builtin("string-range", string_range);
    interpreter.add_builtin("string->u8vector", string_into_u8vector);
    interpreter.add_builtin("u8vector->string", u8vector_into_string);
    interpreter.add_builtin("u8vector-invalid-char-index", u8vector_invalid_char_index);
    interpreter.add_builtin("string-char-boundary?", is_string_char_boundary);
    interpreter.add_builtin("string-split", string_split);
    interpreter.add_builtin("symbol->string", symbol_into_string);
    interpreter.add_builtin("string->symbol", string_into_symbol);
}

fn string_into_list(interpreter: &mut Interpreter) -> exec::Result<()> {
    let mut s = interpreter.stack.pop::<String>()?;
    let chars: Vec<char> = s.drain(..).collect();
    let l = List::from(chars);
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(l));
    Ok(())
}

fn string_length(interpreter: &mut Interpreter) -> exec::Result<()> {
    let len = {
        let s = interpreter.stack.ref_at::<String>(0)?;
        s.chars().count()
    };
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(len)));
    Ok(())
}

fn string_get(interpreter: &mut Interpreter) -> exec::Result<()> {
    let idx = interpreter.stack.pop::<Number>()?.cast::<isize>()?;
    let ch = {
        let s = interpreter.stack.ref_at::<String>(0)?;
        if idx >= 0 {
            match s.chars().nth(idx as usize) {
                None => return Err(error::OutOfRange(0, (s.chars().count() - 1) as isize, idx).into()),
                Some(s) => s,
            }
        } else {
            // No nth for the other end of an iterator
            let mut c = -idx as usize;
            let mut chars = s.chars();
            'chr: loop {
                c -= 1;
                if let Some(ch) = chars.next_back() {
                    if c == 0 {
                        break 'chr ch;
                    }
                } else {
                    let c = s.chars().count() as isize;
                    return Err(error::OutOfRange(-c, c - 1, idx).into());
                }
            }
        }
    };
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(ch));
    Ok(())
}

fn string_set(interpreter: &mut Interpreter) -> exec::Result<()> {
    Err(error::NotImplemented().into())
}

fn string_compare(interpreter: &mut Interpreter) -> exec::Result<()> {
    Err(error::NotImplemented().into())
}

fn string_compare_ci(interpreter: &mut Interpreter) -> exec::Result<()> {
    Err(error::NotImplemented().into())
}

fn string_upcase(interpreter: &mut Interpreter) -> exec::Result<()> {
    Err(error::NotImplemented().into())
}

fn string_downcase(interpreter: &mut Interpreter) -> exec::Result<()> {
    Err(error::NotImplemented().into())
}

fn string_append(interpreter: &mut Interpreter) -> exec::Result<()> {
    let app = interpreter.stack.pop::<String>()?;
    let s = interpreter.stack.top_mut::<String>()?;
    s.push_str(app.as_str());
    Ok(())
}

fn string_push(interpreter: &mut Interpreter) -> exec::Result<()> {
    let c = interpreter.stack.pop::<char>()?;
    let s = interpreter.stack.top_mut::<String>()?;
    s.push(c);
    Ok(())
}

fn string_pop(interpreter: &mut Interpreter) -> exec::Result<()> {
    let c = {
        let s = interpreter.stack.top_mut::<String>()?;
        s.pop().ok_or(StringEmpty())?
    };
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(c));
    Ok(())
}

fn string_range(interpreter: &mut Interpreter) -> exec::Result<()> {
    Ok(())
}

fn string_into_u8vector(interpreter: &mut Interpreter) -> exec::Result<()> {
    let s = interpreter.stack.pop::<String>()?;
    let v: U8Vector = s.into_bytes().into();
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(v));
    Ok(())
}

fn u8vector_into_string(interpreter: &mut Interpreter) -> exec::Result<()> {
    let v = interpreter.stack.pop::<U8Vector>()?;
    let s: String = String::from_utf8(v.into()).map_err(|_| InvalidString())?;
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(s));
    Ok(())
}

fn u8vector_invalid_char_index(interpreter: &mut Interpreter) -> exec::Result<()> {
    let idx = {
        let v = interpreter.stack.ref_at::<U8Vector>(0)?;
        match str::from_utf8(v.inner()) {
            Ok(_) => None,
            Err(e) => Some(e.valid_up_to()),
        }
    };
    let source = interpreter.current_source();
    match idx {
        Some(i) => interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(i))),
        None => interpreter.stack.push(Datum::build().with_source(source).ok(false)),
    }
    Ok(())
}

fn is_string_char_boundary(interpreter: &mut Interpreter) -> exec::Result<()> {
    let is_boundary = {
        let idx = interpreter.stack.ref_at::<Number>(0)?.cast::<usize>()?;
        let s = interpreter.stack.ref_at::<String>(1)?;
        s.as_str().is_char_boundary(idx)
    };
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(is_boundary));
    Ok(())
}

fn string_split(interpreter: &mut Interpreter) -> exec::Result<()> {
    Ok(())
}

fn symbol_into_string(interpreter: &mut Interpreter) -> exec::Result<()> {
    let (sym, source) = interpreter.stack.pop_source::<Symbol>()?;
    interpreter.stack.push(Datum::build().with_source(source).ok::<String>(sym.to_string()));
    Ok(())
}

fn string_into_symbol(interpreter: &mut Interpreter) -> exec::Result<()> {
    let (a, source) = interpreter.stack.pop_source::<String>()?;
    interpreter.stack.push(Datum::build().with_source(source).ok(a));
    Ok(())
}

#[derive(Debug)]
pub struct BadStringIndex(pub isize);
impl Error for BadStringIndex {}

impl fmt::Display for BadStringIndex {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Not a valid string index: {}", self.0)
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct StringEmpty();
impl Error for StringEmpty {}

impl fmt::Display for StringEmpty {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Empty string")
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct InvalidString();
impl Error for InvalidString {}

impl fmt::Display for InvalidString {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Not a valid string")
    }
}

