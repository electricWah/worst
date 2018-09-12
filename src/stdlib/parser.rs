
use parser::*;
use data::*;
use data::error;
use interpreter::Interpreter;
use interpreter::command::*;
use interpreter::exec;
use stdlib::enumcommand::*;
use stdlib::combo::ComboValue;

pub fn install(interpreter: &mut Interpreter) {
    ParseOp::install(interpreter);
}

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ParseOp {
    // New rule with name and accept rules
    ParserNewRule,
    IsParserRule,
    // CharClass and Combo<char> ops
    CharClassJustChar,
    CharClassWhitespace,
    CharClassAlpha,
    CharClassNumeric,
    CharClassSymbol,
    CharClassEof,
    IsCharClass,
    // Instructions mirroring parser::ReaderCommand
    ParserAcceptInput,
    ParserAcceptState,
    ParserSetState,
    ParserStartToken,
    ParserSetTokenTag,
    ParserSetTokenType,
    ParserAppendToken,
    ParserFinishToken,
    ParserPrependDatum,

    // CurrentSourcePosition,
    // ParserSetFile,

    // Put the rule in
    ParserSaveRule,
    // leave this at the end
    ParserDeleteRule,
}

impl EnumCommand for ParseOp {
    fn as_str(&self) -> &str {
        use self::ParseOp::*;
        match self {
            ParserNewRule => "parser-new-rule",
            IsParserRule => "parser-rule?",
            CharClassJustChar => "char-class-just",
            CharClassWhitespace => "char-class-whitespace",
            CharClassAlpha => "char-class-alpha",
            CharClassNumeric => "char-class-numeric",
            CharClassSymbol => "char-class-symbol",
            CharClassEof => "char-class-eof",
            IsCharClass => "char-class?",
            ParserAcceptInput => "parser-accept-input",
            ParserAcceptState => "parser-accept-state",
            ParserSetState => "parser-set-state",
            ParserStartToken => "parser-start-token",
            ParserSetTokenTag => "parser-set-token-tag",
            ParserSetTokenType => "parser-set-token-type",
            ParserAppendToken => "parser-append-token",
            ParserFinishToken => "parser-finish-token",
            ParserPrependDatum => "parser-prepend-datum",
            ParserSaveRule => "parser-save-rule",
            ParserDeleteRule => "parser-delete-rule",
        }
    }
    fn last() -> Self { ParseOp::ParserDeleteRule }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

impl Command for ParseOp {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        debug!("ParseOp: {:?}", self);
        use self::ParseOp::*;
        match self {
            &ParserNewRule => {
                let name = interpreter.stack.pop::<String>()?;
                let arm = ReaderArm::new(name);
                interpreter.stack.push(Datum::build().with_source(source).ok(arm));
            },
            &IsParserRule => {
                let r = interpreter.stack.type_predicate::<ReaderArm>(0)?;
                interpreter.stack.push(Datum::build().with_source(source).ok(r));
            },
            &CharClassJustChar => {
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

            &ParserAcceptInput => {
                let class = interpreter.stack.pop::<ComboValue>()?.into_combo::<CharClass>()?;
                let mut arm = interpreter.stack.top_mut::<ReaderArm>()?;
                arm.set_accept_input(class);
            },
            &ParserAcceptState => {
                let combo = interpreter.stack.pop::<ComboValue>()?.into_combo::<Symbol>()?;
                let mut arm = interpreter.stack.top_mut::<ReaderArm>()?;
                arm.set_accept_state(combo);
            },

            &ParserSetState => {
                let state = interpreter.stack.pop::<Symbol>()?;
                let mut arm = interpreter.stack.top_mut::<ReaderArm>()?;
                arm.push_run(ReaderInstruction::set_state(state));
            },

            &ParserStartToken => {
                let mut arm = interpreter.stack.top_mut::<ReaderArm>()?;
                arm.push_run(ReaderInstruction::start_token());
            },

            &ParserSetTokenTag => {
                let tag = interpreter.stack.pop::<String>()?;
                let mut arm = interpreter.stack.top_mut::<ReaderArm>()?;
                arm.push_run(ReaderInstruction::set_tag(tag));
            },

            &ParserSetTokenType => {
                let state = interpreter.stack.pop::<Symbol>()?;
                let ty = TokenType::from_symbol(state)?;
                let mut arm = interpreter.stack.top_mut::<ReaderArm>()?;
                arm.push_run(ReaderInstruction::set_type(ty));
            },

            &ParserAppendToken => {
                let mut arm = interpreter.stack.top_mut::<ReaderArm>()?;
                arm.push_run(ReaderInstruction::append_token());
            },

            &ParserFinishToken => {
                let mut arm = interpreter.stack.top_mut::<ReaderArm>()?;
                arm.push_run(ReaderInstruction::finish_token());
            },

            &ParserPrependDatum => {
                let d = interpreter.stack.pop_datum()?;
                let mut arm = interpreter.stack.top_mut::<ReaderArm>()?;
                arm.push_run(ReaderInstruction::prepend_datum(d));
            },

            &ParserSaveRule => {
                let arm = interpreter.stack.pop::<ReaderArm>()?;
                interpreter.reader_mut().add_rule(arm);
            },

            // &ParseOp::DeleteRule,

            _ => return Err(error::NotImplemented().into()),
        }
        Ok(())
    }
}

impl StaticType for ReaderArm {
    fn static_type() -> Type {
        Type::new("reader-arm")
    }
}
impl ValueShow for ReaderArm {}
impl ValueDebugDescribe for ReaderArm {}
impl ValueHash for ReaderArm {}
impl DefaultValueEq for ReaderArm {}
impl DefaultValueClone for ReaderArm {}
impl Value for ReaderArm {}

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

impl TokenType {
    fn from_symbol(sym: Symbol) -> exec::Result<Self> {
        match sym.as_ref() {
            "symbol" => Ok(TokenType::Symbol),
            "string" => Ok(TokenType::String),
            "rational" => Ok(TokenType::Rational),
            "start-list" => Ok(TokenType::StartList),
            "end-list" => Ok(TokenType::EndList),
            _ => Err(error::NotDefined().into()),
        }
    }
}

