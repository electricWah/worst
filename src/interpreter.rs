
pub mod code;
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
use self::code::*;
use self::stack::Stack;

pub struct Interpreter {
    pub stack: Stack,
    pub context: Context,
    builtins: builtin::BuiltinLookup,
    history: VecDeque<Symbol>,
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
            history: Default::default(),
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

    pub fn define<S: Into<Symbol>, C: Into<Code>>(&mut self, name: S, code: C) {
        self.context.env_mut().define(name, code);
    }

    pub fn define_type_predicate<T: IsType + Value>(&mut self, name: &str) {
        self.add_builtin(name, type_predicate::<T>);
    }

    pub fn add_builtin<S: Into<Symbol>, B: 'static + builtin::BuiltinFn>(&mut self, name: S, builtin: B) {
        let name = name.into();
        let builtin_ref = self.builtins.add(name.clone(), builtin);
        self.context.env_mut().define(name, builtin_ref);
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

    fn run_available(&mut self) -> exec::Result<()> {
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

    pub fn eval_run(&mut self) -> exec::Result<()> {
        self.run_available()
    }

    pub fn eval_code(&mut self, code: &Code) -> exec::Result<()> {
        match code.value() {
            Instruction::Builtin(ref b) => {
                let mut b = self.builtins.lookup(b);
                b.call(self)?;
            },
            Instruction::Definition(ref def) => {
                self.context.push_def(def);
                // self.run_available()?;
            },
        }
        Ok(())
    }

    pub fn resolve_symbol(&self, r: &Symbol) -> Option<Code> {
        self.context.resolve(r).cloned()
    }

    fn push_history(&mut self, h: &Symbol) {
        self.history.push_back(h.clone());
        while self.history.len() > 20 {
            self.history.pop_front();
        }
    }

    pub fn history(&self) -> std::collections::vec_deque::Iter<Symbol> {
        self.history.iter()
    }

    pub fn eval_symbol(&mut self, r: &Symbol) -> exec::Result<()> {
        let code = self.resolve_symbol(r).ok_or_else(|| error::NotDefined(r.clone()))?;
        let res = self.eval_code(&code);
        self.push_history(&r);
        if let Err(e) = res {
            // Hack to show where the error occurred
            Err(e)?;
        }
        Ok(())
    }

    fn push_eval(&mut self, d: Datum) -> exec::Result<()> {
        self.context.add_code(d);
        self.run_available()
    }

}

impl Interpreter {
    // Should be AsRef<Path>
    // This is manageable as a completely hosted function
    pub fn eval_file(&mut self, path: &str) -> exec::Result<()> {
        use ::std::fs::File;
        use ::std::io::Read;
        let mut file = File::open(&path).map_err(error::StdIoError::new)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(error::StdIoError::new)?;

        let mut parser = Parser::new(contents.chars().into_iter()).with_file(path);

        while let Some(datum) = parser.next()? {
            self.push_eval(datum)?;
        }
        Ok(())
    }
}

