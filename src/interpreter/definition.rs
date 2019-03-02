
use crate::data::*;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Definition {
    name: Option<Symbol>,
    body: Vec<Datum>,
}

impl Definition {
    pub fn new(body: Vec<Datum>) -> Self {
        Definition {
            name: None,
            body,
        }
    }

    pub fn with_name(mut self, name: Symbol) -> Self {
        self.set_name(name);
        self
    }

    pub fn set_name(&mut self, name: Symbol) {
        self.name = Some(name);
    }

    pub fn program(&self) -> &Vec<Datum> {
        &self.body
    }

    pub fn name(&self) -> Option<&Symbol> {
        self.name.as_ref()
    }
    pub fn take_name(&mut self) -> Option<Symbol> {
        self.name.take()
    }
}

impl StaticType for Definition {
    fn static_type() -> Type {
        Type::new("definition")
    }
}

impl ValueEq for Definition {}
impl ValueHash for Definition {}
impl ValueShow for Definition {}
impl DefaultValueClone for Definition {}
impl ValueDebugDescribe for Definition {}
impl Value for Definition {}

