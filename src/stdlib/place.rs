
use std::fmt;
use std::mem;
use std::rc::Rc;
use std::cell::RefCell;

use crate::data::*;
use crate::parser::*;
use crate::interpreter::Interpreter;
use crate::interpreter::command::*;
use crate::interpreter::exec;
use crate::stdlib::enumcommand::*;

#[derive(Clone, Debug)]
struct Place(Rc<RefCell<Datum>>);

impl Place {
    fn new(d: Datum) -> Self {
        Place(Rc::new(RefCell::new(d)))
    }
    fn swap(&mut self, mut other: Datum) -> Datum {
        mem::swap(&mut *self.0.borrow_mut(), &mut other);
        other
    }
}

impl PartialEq for Place {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for Place {}

impl IsType for Place {
    fn get_type() -> Type {
        Type::new("place")
    }
}
impl HasType for Place {
    fn type_of(&self) -> Type {
        Type::new(format!("place({})", self.0.borrow().type_of()))
    }
}

impl ValueDescribe for Place {
    fn fmt_describe(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Place: ")?;
        self.0.borrow().fmt_describe(fmt)
    }
}

impl DefaultValueClone for Place {}
impl DefaultValueEq for Place {}
impl ValueHash for Place {}
impl ValueShow for Place {}
impl Value for Place {}

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PlaceOp {
    MakePlace,
    PlaceSwap,
    IsPlace,
}

impl EnumCommand for PlaceOp {
    fn as_str(&self) -> &str {
        use self::PlaceOp::*;
        match self {
            MakePlace => "make-place",
            PlaceSwap => "place-swap",
            IsPlace => "place?",
        }
    }
    fn last() -> Self { PlaceOp::IsPlace }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

pub fn install(interpreter: &mut Interpreter) {
    PlaceOp::install(interpreter);
}

impl Command for PlaceOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::PlaceOp::*;
        match self {
            MakePlace => {
                let d = interpreter.stack.pop_datum()?;
                let place = Place::new(d);
                interpreter.stack.push(Datum::build().with_source(source).ok(place));
            },
            PlaceSwap => {
                let d = interpreter.stack.pop_datum()?;
                let d = {
                    let place = interpreter.stack.top_mut::<Place>()?;
                    place.swap(d)
                };
                interpreter.stack.push(d);
            },
            IsPlace => {
                let ok = interpreter.stack.type_predicate::<Place>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(ok));
            },
        }
        Ok(())
    }
}



