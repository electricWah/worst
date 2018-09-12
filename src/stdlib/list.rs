
use data::*;
use parser::*;
use interpreter::Interpreter;
use interpreter::command::*;
use interpreter::exec;
use stdlib::enumcommand::*;

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ListOp {

    ListPushHead,
    ListPushTail,
    ListPopHead,
    ListPopTail,
    ListAppend,
    ListLength,
    // ListGet,
    // ListSet,
    ListSwap,
    // ListTakeWithHead,
    // ListTakeWithTail,

    IsList,
}

impl EnumCommand for ListOp {
    fn as_str(&self) -> &str {
        use self::ListOp::*;
        match self {
            ListPushHead => "list-push-head",
            ListPushTail => "list-push-tail",
            ListPopHead => "list-pop-head",
            ListPopTail => "list-pop-tail",
            ListAppend => "list-append",
            ListLength => "list-length",
            // ListGet => "list-get",
            // ListSet => "list-set",
            ListSwap => "list-swap",
            // ListTakeWithHead => "list-take/head",
            // ListTakeWithTail => "list-take/tail",
            IsList => "list?",
        }
    }
    fn last() -> Self { ListOp::IsList }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

pub fn install(interpreter: &mut Interpreter) {
    ListOp::install(interpreter);
}

impl Command for ListOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::ListOp::*;
        match self {
            ListLength => {
                let len = { interpreter.stack.ref_at::<List>(0)?.len() };
                interpreter.stack.push(Datum::build().with_source(source).ok(Number::exact(len)));
            },
            ListPushHead => {
                let a = interpreter.stack.pop_datum()?;
                let mut l = interpreter.stack.top_mut::<List>()?;
                l.push_head(a);
            },
            ListPushTail => {
                let a = interpreter.stack.pop_datum()?;
                let mut l = interpreter.stack.top_mut::<List>()?;
                l.push_tail(a);
            },
            ListPopHead => {
                let a = {
                    let mut l = interpreter.stack.top_mut::<List>()?;
                    l.pop_head().ok_or(exec::Exception::from(error::ListEmpty()))?
                };
                interpreter.stack.push(a);
            },
            ListPopTail => {
                let a = {
                    let mut l = interpreter.stack.top_mut::<List>()?;
                    l.pop_tail().ok_or(exec::Exception::from(error::ListEmpty()))?
                };
                interpreter.stack.push(a);
            },
            ListAppend => {
                let b = interpreter.stack.pop::<List>()?;
                let mut a = interpreter.stack.top_mut::<List>()?;
                a.append(b);
            },
            ListSwap => {
                let j = interpreter.stack.pop::<Number>()?.cast::<usize>()?;
                let i = interpreter.stack.pop::<Number>()?.cast::<usize>()?;
                let mut lis = interpreter.stack.top_mut::<List>()?;
                let len = lis.len();
                if i > len {
                    Err(error::OutOfRange(0, len as isize - 1, i as isize))?;
                }
                if j > len {
                    Err(error::OutOfRange(0, len as isize - 1, j as isize))?;
                }
                lis.swap(i, j);
            },
            IsList => {
                let ok = interpreter.stack.type_predicate::<List>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(ok));
            },
        }
        Ok(())
    }
}


