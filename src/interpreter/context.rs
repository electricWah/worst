
use std::mem;
use std::collections::VecDeque;
use crate::interpreter::code::*;
use crate::interpreter::exec;
use crate::parser::Source;
use crate::data::*;
use crate::interpreter::env::*;

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
    name: Option<String>,
}

impl Context {

    pub fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }

    pub fn set_name<S: Into<Option<String>>>(&mut self, name: S) {
        self.name = name.into();
    }

    /// Go back up to root context and remove code
    pub fn reset(&mut self) {
        while let Some(mut p) = self.parent.take() {
            mem::swap(self, &mut p);
        }
        self.code.clear();
        self.children.clear();
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    fn into_child_context(&mut self, mut child: Context) {
        let mut swap = Context::default();
        mem::swap(self, &mut swap);
        child.parent = Some(Box::new(swap));
        mem::swap(self, &mut child);
    }

    // Only for interpreter to push place of current error (hack)
    pub fn push_source<P: Into<Option<Source>>>(&mut self, source: P) {
        let mut ctx = Context::default();
        ctx.source = source.into();
        self.into_child_context(ctx);
    }

    pub fn push_def<P: Into<Option<Source>>>(&mut self, source: P, def: &Definition) {
        let source = source.into();
        // TCO here
        if self.is_root() || self.code.len() > 0 {
            self.into_child_context(Default::default());
        }
        self.source = source;
        self.code = def.program().clone().into_iter().collect();
    }
    
    // Become parent and add old self as child
    pub fn uplevel(&mut self, _source: Option<Source>) -> exec::Result<()> {
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

    pub fn next_top(&mut self) -> Option<Datum> {
        self.code.pop_front()
    }

    pub fn next(&mut self) -> exec::Result<Option<Datum>> {
        while let Some(child) = self.children.pop() {
            self.into_child_context(child);
        }
        if let Some(n) = self.code.pop_front() {
            return Ok(Some(n));
        }
        match self.parent.take() {
            None => Ok(None),
            Some(mut p) => {
                mem::swap(self, &mut p);
                self.next()
            },
        }
    }

    pub fn add_code(&mut self, code: Datum) {
        self.code.push_back(code);
    }

    pub fn current_defines(&self) -> impl Iterator<Item=&Symbol> {
        self.env.0.keys()
    }

    pub fn resolve(&self, name: &Symbol) -> Option<&Code> {
        if let Some(def) = self.env.0.get(name) {
            return Some(def);
        }
        if let Some(ref p) = self.parent.as_ref() {
            return p.resolve(name);
        }
        return None;
    }

    pub fn stack_sources(&self) -> Vec<Option<Source>> {
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

    pub fn env_mut(&mut self) -> &mut Env {
        &mut self.env
    }
    pub fn env(&self) -> &Env {
        &self.env
    }

}

