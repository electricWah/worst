
// pub mod definition;
mod context;
pub mod exec;
mod stack;
pub mod builtin;
pub mod env;

use std::collections::VecDeque;

use crate::data::*;
use crate::parser::*;
use crate::data::error;
use self::context::*;
// use self::definition::*;
use self::stack::Stack;

pub struct Interpreter {
    pub stack: Stack,
    pub context: Context,
    builtins: builtin::BuiltinLookup,
    quoting: bool,
}

fn type_predicate<T: IsType + Value>(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = interpreter.stack.type_predicate::<T>(0)?;
    interpreter.stack.push(Datum::new(r));
    Ok(())
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            stack: Default::default(),
            context: Context::default(),
            builtins: Default::default(),
            quoting: false,
        }
    }

    pub fn quoting(&self) -> bool {
        self.quoting
    }

    pub fn set_quoting(&mut self, q: bool) {
        self.quoting = q;
    }

    pub fn quote_next(&mut self) {
        self.quoting = true;
    }

    pub fn define<S: Into<Symbol>, D: Into<Vec<Datum>>>(&mut self, name: S, def: D) {
        self.context.env_mut().define(name, def);
    }

    pub fn define_type_predicate<T: IsType + Value>(&mut self, name: &str) {
        self.add_builtin(name, type_predicate::<T>);
    }

    pub fn add_builtin<S: Into<Symbol>, B: 'static + builtin::BuiltinFn>(&mut self, name: S, builtin: B) {
        let name = name.into();
        self.builtins.add(name.clone(), builtin);
    }

    // pub fn evaluate<A: builtin::BuiltinFnArgs, R: builtin::BuiltinFnRets, F: FnMut(A) -> exec::Result<R>>(&mut self, mut f: F) -> exec::Result<()> {
    //     use self::builtin::BuiltinFnRet;
    //     let args = A::extract(self)?;
    //     for r in f(args)?.into_datums().into_iter() {
    //         self.stack.push(r.into_datum());
    //     }
    //     Ok(())
    // }

    pub fn clear(&mut self) {
        self.context.reset();
        self.quoting = false;
    }

    pub fn env_mut(&mut self) -> &mut env::Env {
        self.context.env_mut()
    }
    pub fn env(&self) -> &env::Env {
        self.context.env()
    }
}

impl Interpreter {

    pub fn read_next(&mut self) -> exec::Result<Option<Datum>> {
        if self.quoting {
            Ok(self.context.next_top())
        } else {
            self.context.next()
        }
    }

    fn eval_result(&mut self) -> exec::Result<()> {
        while let Some(d) = self.read_next()? {
            if self.quoting {
                self.quoting = false;
                self.stack.push(d);
            } else if let Ok(r) = d.value_ref::<Symbol>() {
                self.eval_symbol(&r)?;
            } else {
                self.stack.push(d);
            }
        }
        Ok(())
    }

    pub fn eval_run(&mut self) {
        while let Err(e) = self.eval_result() {
            // eprintln!("{:?}", e);
            self.stack.push(Datum::new(e));
            if let Some(handler) = self.resolve_symbol(&"%%failure".into()) {
                self.context.push_def(handler);
            }
        }
    }

    pub fn resolve_symbol(&self, r: &Symbol) -> Option<Vec<Datum>> {
        self.context.resolve(r).map(|c| c.iter().map(Clone::clone).collect())
    }

    pub fn eval_definition<D: Into<VecDeque<Datum>>>(&mut self, def: D) -> exec::Result<()> {
        self.context.push_def(def);
        Ok(())
        // self.eval_run()
    }

    pub fn eval_builtin(&mut self, r: &Symbol) -> exec::Result<()> {
        match self.builtins.lookup(r) { 
            Some(mut b) => {
                b.call(self)?;
            },
            None => {
                Err(error::NotDefined(r.clone()))?;
            },
        }
        Ok(())
    }

    pub fn eval_symbol(&mut self, r: &Symbol) -> exec::Result<()> {
        match self.resolve_symbol(r) {
            Some(def) => {
                self.context.push_def(def);
            },
            None => {
                self.eval_builtin(r)?;
            },
        }
        Ok(())
    }

}

impl Interpreter {
    // These have the same code but shrug
    pub fn read_file(path: &str) -> exec::Result<Vec<Datum>> {
        use ::std::fs::File;
        use ::std::io::Read;
        let mut file = File::open(&path).map_err(error::StdIoError::new)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(error::StdIoError::new)?;

        let mut parser = Parser::new(contents.chars().into_iter()).with_file(path);

        let mut r = vec![];
        while let Some(d) = parser.next()? {
            r.push(d);
        }
        Ok(r)
    }

    // Should be AsRef<Path>
    // This is manageable as a completely hosted function
    pub fn load_file(&mut self, path: &str) -> exec::Result<()> {
        use ::std::fs::File;
        use ::std::io::Read;
        let mut file = File::open(&path).map_err(error::StdIoError::new)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(error::StdIoError::new)?;

        let mut parser = Parser::new(contents.chars().into_iter()).with_file(path);

        while let Some(datum) = parser.next()? {
            self.context.add_code(datum);
        }

        // self.eval_run()?;
        Ok(())
    }
}

