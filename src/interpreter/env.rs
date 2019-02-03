
use std::collections::HashMap;
use crate::interpreter::code::*;
use crate::data::*;

#[derive(Default, Debug)]
pub struct Env(pub HashMap<Symbol, Code>);

impl Env {
    pub fn define<S: Into<Symbol>, C: Into<Code>>(&mut self, name: S, code: C) {
        self.0.insert(name.into(), code.into());
    }

    pub fn undefine(&mut self, name: &Symbol) -> Option<Code> {
        debug!("undefine");
        self.0.remove(name)
    }

    pub fn is_defined(&self, name: &Symbol) -> bool {
        self.0.contains_key(name)
    }

    pub fn get_definition(&mut self, name: &Symbol) -> Option<Code> {
        self.0.get(name).cloned()
    }

    pub fn current_defines(&self) -> impl Iterator<Item=&Symbol> {
        self.0.keys()
    }
}

