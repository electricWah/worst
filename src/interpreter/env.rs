
use std::collections::HashMap;
// use crate::interpreter::definition::*;
use crate::data::*;

#[derive(Default, Debug)]
pub struct Env(HashMap<Symbol, Vec<Datum>>);

impl Env {
    pub fn define<S: Into<Symbol>, D: Into<Vec<Datum>>>(&mut self, name: S, def: D) {
        self.0.insert(name.into(), def.into());
    }

    pub fn undefine(&mut self, name: &Symbol) -> Option<Vec<Datum>> {
        self.0.remove(name)
    }

    pub fn is_defined(&self, name: &Symbol) -> bool {
        self.0.contains_key(name)
    }

    pub fn get_definition(&self, name: &Symbol) -> Option<&[Datum]> {
        self.0.get(name).map(Vec::as_slice)
    }

    pub fn current_defines(&self) -> impl Iterator<Item=&Symbol> {
        self.0.keys()
    }
}

