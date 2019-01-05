
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use crate::data::*;
use crate::interpreter::Interpreter;

#[derive(Clone)]
pub enum InterpRef {
    Current,
    Ref(Rc<RefCell<Interpreter>>),
}

impl PartialEq for InterpRef {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InterpRef::Current, InterpRef::Current) => true,
            (InterpRef::Ref(a), InterpRef::Ref(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Eq for InterpRef {}

impl StaticType for InterpRef {
    fn static_type() -> Type {
        Type::new("interpreter")
    }
}

impl ValueDescribe for InterpRef {
    fn fmt_describe(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_show(fmt)
    }
}

impl ValueShow for InterpRef {}
impl ValueHash for InterpRef {}
impl ValueDefaults for InterpRef {}
impl Value for InterpRef {}


impl InterpRef {

    pub fn current() -> Self {
        InterpRef::Current
    }

}

impl From<Interpreter> for InterpRef {
    fn from(i: Interpreter) -> InterpRef {
        InterpRef::Ref(Rc::new(RefCell::new(i)))
    }
}

