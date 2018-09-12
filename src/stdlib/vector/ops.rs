
use data::*;
use parser::*;
use interpreter::Interpreter;
use interpreter::command::*;
use interpreter::exec;
use stdlib::enumcommand::*;

use stdlib::vector::data::*;

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum U8VectorOp {
    MakeU8Vector,
    U8VectorLength,
    U8VectorGet,
    U8VectorSet,
    U8VectorToList,
    U8VectorSplit,
    U8VectorTruncate,
    U8VectorExtend,
    U8VectorAppend,
    U8VectorPush,
    ListToU8Vector,
    IsU8Vector,
}

impl EnumCommand for U8VectorOp {
    fn as_str(&self) -> &str {
        use self::U8VectorOp::*;
        match self {
            MakeU8Vector => "make-u8vector",
            U8VectorLength => "u8vector-length",
            U8VectorGet => "u8vector-get",
            U8VectorSet => "u8vector-set",
            U8VectorSplit => "u8vector-split",
            U8VectorTruncate => "u8vector-truncate",
            U8VectorExtend => "u8vector-extend",
            U8VectorAppend => "u8vector-append",
            U8VectorPush => "u8vector-push",
            U8VectorToList => "u8vector->list",
            ListToU8Vector => "list->u8vector",
            IsU8Vector => "u8vector?",
        }
    }
    fn last() -> Self { U8VectorOp::IsU8Vector }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

impl Command for U8VectorOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::U8VectorOp::*;
        match self {
            &MakeU8Vector => {
                let len = interpreter.stack.pop::<Number>()?.cast::<usize>()?;
                let fill = interpreter.stack.pop::<Number>()?.cast::<u8>()?;
                let vec = U8Vector::fill(len, fill);
                interpreter.stack.push(Datum::build().with_source(source).ok(vec));
            },
            &U8VectorLength => {
                let len = interpreter.stack.ref_at::<U8Vector>(0)?.len();
                interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(len)));
            },
            &U8VectorGet => {
                let idx = interpreter.stack.ref_at::<Number>(0)?.cast::<usize>()?;
                let (len, got) = {
                    let vec = interpreter.stack.ref_at::<U8Vector>(1)?;
                    (vec.len(), vec.inner().get(idx).cloned())
                };
                match got {
                    Some(v) => {
                        interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(v)));
                    },
                    None => return Err(error::OutOfRange(0, (len - 1) as isize, idx as isize).into()),
                }
            },
            &U8VectorSet => {
                let idx = interpreter.stack.pop::<Number>()?.cast::<usize>()?;
                let val = interpreter.stack.pop::<Number>()?.cast::<u8>()?;
                let bad = {
                    let mut vec = interpreter.stack.top_mut::<U8Vector>()?.inner_mut();
                    let len = vec.len();
                    if let Some(x) = vec.get_mut(idx) {
                        *x = val;
                        None
                    } else {
                        Some(len)
                    }
                };
                if let Some(len) = bad {
                    return Err(error::OutOfRange(0, (len - 1) as isize, idx as isize).into());
                }
            },
            &U8VectorTruncate => {
                let len = interpreter.stack.pop::<Number>()?.cast::<usize>()?;
                let mut vec = interpreter.stack.top_mut::<U8Vector>()?;
                vec.inner_mut().truncate(len);
                vec.inner_mut().shrink_to_fit();
            },
            &U8VectorExtend => {
                let len = interpreter.stack.pop::<Number>()?.cast::<usize>()?;
                let val = interpreter.stack.pop::<Number>()?.cast::<u8>()?;
                let mut vec = interpreter.stack.top_mut::<U8Vector>()?;
                let clen = vec.len();
                vec.inner_mut().resize(len + clen, val);
            },
            &U8VectorAppend => {
                let mut b = interpreter.stack.pop::<U8Vector>()?;
                let mut a = interpreter.stack.top_mut::<U8Vector>()?;
                a.inner_mut().append(b.inner_mut());
            },
            &U8VectorPush => {
                let v = interpreter.stack.pop::<Number>()?.cast::<u8>()?;
                let mut vec = interpreter.stack.top_mut::<U8Vector>()?;
                vec.inner_mut().push(v);
            },
            &IsU8Vector => {
                let ok = interpreter.stack.type_predicate::<U8Vector>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(ok));
            },
            _ => return Err(error::NotImplemented().into()),
        }
        Ok(())
    }
}





