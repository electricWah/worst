
use std::fmt;
use std::mem;
use std::rc::Rc;
use std::cell::RefCell;

use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;

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

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<Place>("place?");
    interpreter.add_builtin("make-place", make_place);
    interpreter.add_builtin("place-swap", place_swap);
}

fn make_place(interpreter: &mut Interpreter) -> exec::Result<()> {
    let d = interpreter.stack.pop_datum()?;
    let place = Place::new(d);
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(place));
    Ok(())
}

fn place_swap(interpreter: &mut Interpreter) -> exec::Result<()> {
    let d = interpreter.stack.pop_datum()?;
    let d = {
        let place = interpreter.stack.top_mut::<Place>()?;
        place.swap(d)
    };
    interpreter.stack.push(d);
    Ok(())
}

