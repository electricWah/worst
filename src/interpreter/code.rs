
use std::rc::Rc;
use crate::data::*;
use crate::parser::*;
use crate::interpreter::builtin::BuiltinRef;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Definition {
    source: Option<Source>,
    body: Vec<Datum>,
}

impl Definition {
    pub fn new(body: Vec<Datum>) -> Self {
        Definition {
            source: None,
            body,
        }
    }

    pub fn with_source<S: Into<Option<Source>>>(mut self, s: S) -> Self {
        self.source = s.into();
        self
    }

    pub fn program(&self) -> &Vec<Datum> {
        &self.body
    }
    pub fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Builtin(BuiltinRef),
    Definition(Rc<Definition>),
    // Reference(String),
    // PushLiteral(Datum),
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        use self::Instruction::*;
        match (self, other) {
            (Builtin(a), Builtin(b)) => a == b,
            (Definition(a), Definition(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Instruction {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Code {
    meta: Option<Datum>,// should this go in definition?
    value: Instruction,
}

impl Code {
    pub fn new(value: Instruction) -> Self {
        Code {
            meta: None,
            value,
        }
    }
    pub fn value(&self) -> &Instruction {
        &self.value
    }

    pub fn meta(&self) -> Option<&Datum> {
        self.meta.as_ref()
    }
    pub fn take_meta(&mut self) -> Option<Datum> {
        self.meta.take()
    }
    pub fn set_meta(&mut self, meta: Datum) {
        self.meta = Some(meta);
    }
    pub fn source(&self) -> Option<Source> {
        match self.value {
            Instruction::Definition(ref d) => d.source().cloned(),
            Instruction::Builtin(_) => None,
        }
    }
}

impl From<Rc<Definition>> for Code {
    fn from(d: Rc<Definition>) -> Self {
        Code::new(Instruction::Definition(d))
    }
}

impl From<BuiltinRef> for Code {
    fn from(b: BuiltinRef) -> Self {
        Code::new(Instruction::Builtin(b))
    }
}

impl From<Definition> for Code {
    fn from(d: Definition) -> Self {
        Code::new(Instruction::Definition(Rc::new(d)))
    }
}

// TODO rename: Code -> Definition; Definition -> ???

impl StaticType for Code {
    fn static_type() -> Type {
        Type::new("definition")
    }
}

impl ValueEq for Code {}
impl ValueHash for Code {}
impl ValueShow for Code {}
impl DefaultValueClone for Code {}
impl ValueDebugDescribe for Code {}
impl Value for Code {}

