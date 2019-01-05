
use std::mem;
use std::collections::{HashMap, VecDeque};
use crate::interpreter::code::*;
use crate::interpreter::exec;
use crate::parser::*;
use crate::data::*;

type Env = HashMap<Symbol, Code>;

// How to find the next thing to execute:
// Become the first child if there is one
// If there is anything in code, that is the next thing.
// Otherwise become the parent
#[derive(Default, Debug)]
pub struct Context {
    source: Option<Source>,
    code: VecDeque<Datum>,
    env: Env,
    parent: Option<Box<Context>>,
    children: Vec<Context>,
    parser: Option<Parser>,
    name: Option<String>,
}

impl Context {

    pub fn with_parser<P: Into<Option<Parser>>>(mut self, parser: P) -> Self {
        self.parser = parser.into();
        self
    }

    pub fn set_parser<P: Into<Option<Parser>>>(&mut self, parser: P) {
        self.parser = parser.into();
    }

    pub fn take_parser(&mut self) -> Option<Parser> {
        self.parser.take()
    }

    pub fn parser(&self) -> Option<&Parser> {
        self.parser.as_ref()
    }

    pub fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }

    pub fn set_name<S: Into<Option<String>>>(&mut self, name: S) {
        self.name = name.into();
    }

    pub fn log(&self, s: &str) {
        debug!("{} name={} parser={} source={} code={} parent={} env={} children={}",
               s,
               self.name.as_ref().unwrap_or(&"".to_string()),
               self.parser.is_some(),
               self.source.is_some(),
               self.code.len(),
               self.parent.is_some(),
               self.env.len(),
               self.children.len());
    }

    /// Go back up to root context and remove code
    pub fn reset(&mut self) {
        self.log("reset");
        while let Some(mut p) = self.parent.take() {
            mem::swap(self, &mut p);
        }
        self.code.clear();
        self.children.clear();
        if let Some(ref mut parser) = self.parser {
            parser.clear();
        }
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    fn into_child_context(&mut self, mut child: Context) {
        self.log("into_child_context");
        let mut swap = Context::default();
        mem::swap(self, &mut swap);
        child.parent = Some(Box::new(swap));
        mem::swap(self, &mut child);
    }

    pub fn push_source<P: Into<Option<Source>>>(&mut self, source: P) {
        debug!("push_source");
        let mut ctx = Context::default();
        ctx.source = source.into();
        self.into_child_context(ctx);
    }

    pub fn push_def<P: Into<Option<Source>>>(&mut self, source: P, def: &Definition) {
        let source = source.into();
        trace!("push_def {:?}", source);
        // TCO here
        if !self.finished() {
            self.into_child_context(Default::default());
        }
        self.source = source;
        self.code = def.program().clone().into_iter().collect();
        self.log("push_def ok");
    }
    
    // Become parent and add old self as child
    pub fn uplevel(&mut self, _source: Option<Source>) -> exec::Result<()> {
        debug!("uplevel");
        let parent = self.parent.take();
        match parent {
            None => Err(error::UplevelInRootContext().into()),
            Some(mut p) => {
                mem::swap(self, &mut p);
                self.children.push(*p);
                Ok(())
            },
        }
    }

    fn finished(&self) -> bool {
        if self.code.len() > 0 {
            return false
        }
        match &self.parser {
            Some(parser) => parser.is_eof(),
            None => true,
        }
    }

    fn current_next(&mut self, reader: &Reader) -> exec::Result<Option<Datum>> {
        match self.take_parser() {
            Some(mut parser) => {
                trace!("current_next parser");
                let r = parser.read_next(reader);
                if !parser.is_eof() {
                    self.set_parser(parser);
                }
                Ok(r?)
            },
            None => {
                trace!("current_next code {:?}", self.code);
                Ok(self.code.pop_front())
            },
        }
    }

    fn next_up(&mut self, reader: &Reader) -> exec::Result<Option<Datum>> {
        // TODO see if need to traverse up parents to get next code
        self.current_next(reader)
    }

    fn next_down(&mut self, reader: &Reader) -> exec::Result<Option<Datum>> {
        while let Some(child) = self.children.pop() {
            trace!("next_code: into child");
            self.into_child_context(child);
        }
        if let Some(code) = self.current_next(reader)? {
            trace!("found code {:?}", code);
            return Ok(Some(code));
        }
        self.log("next_code parent");
        match self.parent.take() {
            None => return Ok(None),
            Some(mut p) => {
                debug!("Leave context");
                if self.env.len() > 0 {
                    let keys: Vec<&Symbol> = self.env.keys().collect();
                    warn!("Dropping defines: {:?}", keys);
                };
                mem::swap(self, &mut p);
                self.next_down(reader)
            },
        }
    }

    pub fn next(&mut self, reader: &Reader, up: bool) -> exec::Result<Option<Datum>> {
        debug!("next: up={}", up);
		if up {
            self.next_up(reader)
		} else {
            self.next_down(reader)
        }
    }

    pub fn add_code(&mut self, code: Datum) {
        debug!("add code {:?}", code);
        self.code.push_back(code);
    }

    pub fn current_defines(&self) -> impl Iterator<Item=&Symbol> {
        self.env.keys()
    }

    pub fn define<S: Into<Symbol>, C: Into<Code>>(&mut self, name: S, code: C) {
        self.env.insert(name.into(), code.into());
    }

    pub fn undefine(&mut self, name: &Symbol) -> Option<Code> {
        debug!("undefine");
        self.env.remove(name)
    }

    pub fn is_defined(&self, name: &Symbol) -> bool {
        self.env.contains_key(name)
    }

    pub fn get_definition(&mut self, name: &Symbol) -> Option<Code> {
        self.env.get(name).cloned()
    }

    pub fn resolve(&self, name: &Symbol) -> Option<&Code> {
        debug!("resolve {:?} in {:?} (env#={})", name, self.name, self.env.len());
        if let Some(def) = self.env.get(name) {
            return Some(def);
        }
        if let Some(ref p) = &self.parent.as_ref() {
            return p.resolve(name);
        }
        return None;
    }

    pub fn stack_sources(&self) -> Vec<Option<Source>> {
        // trace!("{:#?}", self);
        let mut sources = vec![];
        sources.push(self.source.clone());
        let mut r = self;
        while let Some(ref p) = r.parent {
            sources.push(p.source.clone());
            r = &**p;
        }
        sources
    }

    // TODO
    pub fn stack_uplevel_sources(&self) -> Vec<Vec<Option<Source>>> {
        vec![]
    }

}

