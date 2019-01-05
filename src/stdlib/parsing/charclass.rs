
use crate::parser::*;
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::command::*;
use crate::interpreter::exec;
use crate::stdlib::enumcommand::*;

pub fn install(interpreter: &mut Interpreter) {
    CharClassOp::install(interpreter);
}

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CharClassOp {
    CharClassJust,
    CharClassWhitespace,
    CharClassAlpha,
    CharClassNumeric,
    CharClassSymbol,
    CharClassEof,
    IsCharClass,
}

impl EnumCommand for CharClassOp {
    fn as_str(&self) -> &str {
        use self::CharClassOp::*;
        match self {
            CharClassJust => "char-class-just",
            CharClassWhitespace => "char-class-whitespace",
            CharClassAlpha => "char-class-alpha",
            CharClassNumeric => "char-class-numeric",
            CharClassSymbol => "char-class-symbol",
            CharClassEof => "char-class-eof",
            IsCharClass => "char-class?",
        }
    }
    fn last() -> Self { CharClassOp::IsCharClass }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

impl Command for CharClassOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        use self::CharClassOp::*;
        match self {
            &CharClassJust => {
                let chr: CharClass = interpreter.stack.pop::<char>()?.into();
                interpreter.stack.push(Datum::build().with_source(source).ok(chr));
            },
            &CharClassWhitespace => interpreter.stack.push(Datum::build().with_source(source).ok(CharClass::Whitespace)),
            &CharClassAlpha => interpreter.stack.push(Datum::build().with_source(source).ok(CharClass::Alpha)),
            &CharClassNumeric => interpreter.stack.push(Datum::build().with_source(source).ok(CharClass::Numeric)),
            &CharClassSymbol => interpreter.stack.push(Datum::build().with_source(source).ok(CharClass::Symbol)),
            &CharClassEof => interpreter.stack.push(Datum::build().with_source(source).ok(CharClass::Eof)),
            &IsCharClass => {
                let r = interpreter.stack.type_predicate::<CharClass>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
        }
        Ok(())
    }
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

