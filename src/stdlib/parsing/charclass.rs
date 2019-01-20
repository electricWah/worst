
use crate::parser::*;
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;

pub fn install(interpreter: &mut Interpreter) {
    interpreter.define_type_predicate::<CharClass>("char-class?");
    interpreter.add_builtin("char-class-just", char_class_just);
    interpreter.add_builtin("char-class-whitespace", char_class_whitespace);
    interpreter.add_builtin("char-class-alpha", char_class_alpha);
    interpreter.add_builtin("char-class-numeric", char_class_numeric);
    interpreter.add_builtin("char-class-symbol", char_class_symbol);
    interpreter.add_builtin("char-class-eof", char_class_eof);
}

fn char_class_just(interpreter: &mut Interpreter) -> exec::Result<()> {
    let chr: CharClass = interpreter.stack.pop::<char>()?.into();
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(chr));
    Ok(())
}

fn char_class_whitespace(interpreter: &mut Interpreter) -> exec::Result<()> {
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(CharClass::Whitespace));
    Ok(())
}

fn char_class_alpha(interpreter: &mut Interpreter) -> exec::Result<()> {
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(CharClass::Alpha));
    Ok(())
}

fn char_class_numeric(interpreter: &mut Interpreter) -> exec::Result<()> {
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(CharClass::Numeric));
    Ok(())
}

fn char_class_symbol(interpreter: &mut Interpreter) -> exec::Result<()> {
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(CharClass::Symbol));
    Ok(())
}

fn char_class_eof(interpreter: &mut Interpreter) -> exec::Result<()> {
    let source = interpreter.current_source();
    interpreter.stack.push(Datum::build().with_source(source).ok(CharClass::Eof));
    Ok(())
}

impl StaticType for CharClass {
    fn static_type() -> Type {
        Type::new("char-class")
    }
}
impl ValueHash for CharClass {}
impl DefaultValueEq for CharClass {}
impl DefaultValueClone for CharClass {}
impl ValueShow for CharClass {}
impl ValueDebugDescribe for CharClass {}
impl Value for CharClass {}

