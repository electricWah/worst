
use std::fmt;
use crate::parser::*;
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<Reader>("reader?");
    interpreter.add_builtin("make-reader", make_reader);
}

fn make_reader(interpreter: &mut Interpreter) -> exec::Result<()> {
    let name = interpreter.stack.pop::<Symbol>()?;
    let r = Reader::new(name, vec![]);
    interpreter.stack.push(Datum::build().with_source(source).ok(r));
    Ok(())
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

