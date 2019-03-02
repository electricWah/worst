
use std::collections::HashMap;
use crate::interpreter::definition::*;
use crate::data::*;

#[derive(Default, Debug)]
pub struct Env(pub HashMap<Symbol, Definition>);

impl Env {
    pub fn define<S: Into<Symbol>>(&mut self, name: S, def: Definition) {
        self.0.insert(name.into(), def);
    }

    pub fn undefine(&mut self, name: &Symbol) -> Option<Definition> {
        self.0.remove(name)
    }

    pub fn is_defined(&self, name: &Symbol) -> bool {
        self.0.contains_key(name)
    }

    pub fn get_definition(&self, name: &Symbol) -> Option<&Definition> {
        self.0.get(name)
    }

    pub fn current_defines(&self) -> impl Iterator<Item=&Symbol> {
        self.0.keys()
    }
}

