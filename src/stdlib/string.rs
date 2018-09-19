
use std::fmt;
use std::str;
use data::*;
use data::error::*;
use parser::*;
use interpreter::Interpreter;
use interpreter::command::*;
use interpreter::exec;
use stdlib::enumcommand::*;

use stdlib::vector::data::*;

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum StringOp {
    IsString,
    StringToList,
    // StringToListWithRange,
    // StringToListWithRangeByU8,
    StringLength,
    StringLengthByU8,
    StringGet,
    StringGetByU8,
    StringSet,
    StringSetByU8,
    StringCompare,
    StringCompareCaseless,
    StringUpcase,
    StringDowncase,
    StringAppend,
    StringPush,
    StringRange,
    StringRangeByU8,

    // string <-> u8vector
    StringToU8Vector,
    U8VectorToString,
    // returns length of valid utf8 in given u8vector
    // or false if it is entirely valid
    // note: u8vector->string will only work if this gives false
    U8VectorInvalidCharIndex,

    // Other non-r7rs
    IsStringCharBoundary,
    StringSplit,
    StringSplitByU8,
    SymbolToString,
    StringToSymbol,
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
pub struct InvalidString();
impl Error for InvalidString {}

impl fmt::Display for InvalidString {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Not a valid string")
    }
}

impl EnumCommand for StringOp {
    fn as_str(&self) -> &str {
        use self::StringOp::*;
        match self {
            IsString => "string?",
            StringToList => "string->list",
            // StringToListWithRange => "string->list/range",
            // StringToListWithRangeByU8 => "string->list/range/u8",
            StringLength => "string-length",
            StringLengthByU8 => "string-length/u8",
            StringGet => "string-get",
            StringGetByU8 => "string-get/u8",
            StringSet => "string-set",
            StringSetByU8 => "string-set/u8",
            StringCompare => "string-compare",
            StringCompareCaseless => "string-compare/ci",
            StringUpcase => "string-upcase",
            StringDowncase => "string-downcase",
            StringAppend => "string-append",
            StringPush => "string-push",
            StringRange => "string-range",
            StringRangeByU8 => "string-range/u8",
            StringToU8Vector => "string->u8vector",
            U8VectorToString => "u8vector->string",
            U8VectorInvalidCharIndex => "u8vector-invalid-char-index",
            IsStringCharBoundary => "string-char-boundary?",
            StringSplit => "string-split",
            StringSplitByU8 => "string-split/u8",
            SymbolToString => "symbol->string",
            StringToSymbol => "string->symbol",
        }
    }
    fn last() -> Self { StringOp::StringToSymbol }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

pub fn install(interpreter: &mut Interpreter) {
    StringOp::install(interpreter);
}

impl StringOp {
    fn wrap_index(len: usize, idx: isize) -> exec::Result<usize> {
        let len = len as isize;
        if idx.abs() >= len {
            return Err(error::OutOfRange(-len, len, idx).into());
        }
        let udx =
            if idx < 0 {
                (len + idx) as usize
            } else {
                idx as usize
            };
        Ok(udx)
    }
}

impl Command for StringOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::StringOp::*;
        match self {
            IsString => {
                let r = interpreter.stack.type_predicate::<String>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            StringToList => {
                let mut s = interpreter.stack.pop::<String>()?;
                let chars: Vec<char> = s.drain(..).collect();
                let l = List::from(chars);
                interpreter.stack.push(Datum::build().with_source(source).ok(l));
            },
            StringLengthByU8 => {
                let len = {
                    let s = interpreter.stack.ref_at::<String>(0)?;
                    s.len()
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(len)));
            },
            StringLength => {
                let len = {
                    let s = interpreter.stack.ref_at::<String>(0)?;
                    s.chars().count()
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(len)));
            },
            StringGet => {
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
                interpreter.stack.push(Datum::build().with_source(source).ok(ch));
            },
            U8VectorInvalidCharIndex => {
                let idx = {
                    let v = interpreter.stack.ref_at::<U8Vector>(0)?;
                    match str::from_utf8(v.inner()) {
                        Ok(_) => None,
                        Err(e) => Some(e.valid_up_to()),
                    }
                };
                match idx {
                    Some(i) => interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(i))),
                    None => interpreter.stack.push(Datum::build().with_source(source).ok(false)),
                }
            },
            U8VectorToString => {
                let v = interpreter.stack.pop::<U8Vector>()?;
                let s: String = String::from_utf8(v.into()).map_err(|_| InvalidString())?;
                interpreter.stack.push(Datum::build().with_source(source).ok(s));
            },
            StringToU8Vector => {
                let s = interpreter.stack.pop::<String>()?;
                let v: U8Vector = s.into_bytes().into();
                interpreter.stack.push(Datum::build().with_source(source).ok(v));
            },
            IsStringCharBoundary => {
                let is_boundary = {
                    let idx = interpreter.stack.ref_at::<Number>(0)?.cast::<usize>()?;
                    let s = interpreter.stack.ref_at::<String>(1)?;
                    s.as_str().is_char_boundary(idx)
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(is_boundary));
            },
            StringAppend => {
                let app = interpreter.stack.pop::<String>()?;
                let mut s = interpreter.stack.top_mut::<String>()?;
                s.push_str(app.as_str());
            },
            StringPush => {
                let c = interpreter.stack.pop::<char>()?;
                let mut s = interpreter.stack.top_mut::<String>()?;
                s.push(c);
            },
            StringSplitByU8 => {
                let idx = interpreter.stack.pop::<Number>()?.cast::<isize>()?;
                let ss = {
                    let mut s = interpreter.stack.top_mut::<String>()?;
                    let udx = Self::wrap_index(s.len(), idx)?;
                    if !s.as_str().is_char_boundary(udx) {
                        return Err(BadStringIndex(idx).into());
                    }
                    s.split_off(udx)
                };
                interpreter.stack.push(Datum::build().with_source(source).ok(ss));
            },
            SymbolToString => {
                let (sym, source) = interpreter.stack.pop_source::<Symbol>()?;
                interpreter.stack.push(Datum::build().with_source(source).ok::<String>(sym.to_string()));
            },
            StringToSymbol => {
                let (a, source) = interpreter.stack.pop_source::<String>()?;
                interpreter.stack.push(Datum::build().with_source(source).ok(a));
            },
            _ => return Err(error::NotImplemented().into()),
        }
        Ok(())
    }
}



