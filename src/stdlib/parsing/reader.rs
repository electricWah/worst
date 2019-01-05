
use std::fmt;
use crate::parser::*;
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::command::*;
use crate::interpreter::exec;
use crate::stdlib::enumcommand::*;

pub fn install(interpreter: &mut Interpreter) {
    ReaderOp::install(interpreter);
}

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ReaderOp {
    MakeReader,
    IsReader,
}

impl EnumCommand for ReaderOp {
    fn as_str(&self) -> &str {
        use self::ReaderOp::*;
        match self {
            MakeReader => "make-reader",
            IsReader => "reader?",
        }
    }
    fn last() -> Self { ReaderOp::IsReader }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

impl Command for ReaderOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::ReaderOp::*;
        match self {
            MakeReader => {
                let name = interpreter.stack.pop::<Symbol>()?;
                let r = Reader::new(name, vec![]);
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            IsReader => {
                let r = interpreter.stack.type_predicate::<Reader>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
        }
        Ok(())
    }
}

impl StaticType for Reader {
    fn static_type() -> Type {
        Type::new("reader")
    }
}

impl ValueDescribe for Reader {
    fn fmt_describe(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_show(fmt)
    }
}

impl ValueShow for Reader {}
impl ValueHash for Reader {}
impl DefaultValueEq for Reader {}
impl DefaultValueClone for Reader {}
impl Value for Reader {}

