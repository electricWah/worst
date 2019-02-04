
pub mod code;
mod context;
pub mod exec;
mod stack;
pub mod builtin;
pub mod env;

use std::rc::Rc;
use std::fmt;
use crate::data::*;
use crate::parser::*;
use crate::data::error;
use crate::data::error::Error;
use self::context::*;
use self::code::*;
use self::stack::Stack;

pub struct Interpreter {
    pub stack: Stack,
    pub context: Context,
    builtins: builtin::BuiltinLookup,
    evaling_source: Option<Source>, // for builtins
    quoting: bool,
}

// TODO Exception is just for use by outside stuff (main program)
// - rename "_stackless" versions to reflect that they are normal
// - remove Value/Error for Exception
#[derive(Debug, Clone)]
pub struct Exception {
    exception: Rc<exec::Failure>,
    stack_trace: Vec<Option<Source>>,
    uplevel_trace: Vec<Vec<Option<Source>>>,
}

impl Exception {
    fn new<E: Into<Rc<exec::Failure>>>(exception: E,
           (stack_trace, uplevel_trace): (Vec<Option<Source>>, Vec<Vec<Option<Source>>>)) -> Self {
        Exception { exception: exception.into(), stack_trace, uplevel_trace }
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.exception, fmt)?;
        if self.stack_trace.len() + self.uplevel_trace.len() > 0 {
            write!(fmt, "\nStack trace:")?;
        }
        for s in self.stack_trace.iter() {
            write!(fmt, "\n  at ")?;
            if let Some(ref src) = s.as_ref() {
                fmt::Display::fmt(src, fmt)?;
            } else {
                write!(fmt, "<unknown location>")?;
            }
        }
        // if self.uplevel_trace.len() > 0 {
        //     write!(fmt, "\nUplevel stack trace:")?;
        // }
        // for s in self.uplevel_trace.iter() {
        //     write!(fmt, "\n  at ")?;
        //     if let Some(ref src) = s.as_ref() {
        //         fmt::Display::fmt(src, fmt)?;
        //     } else {
        //         write!(fmt, "<unknown location>")?;
        //     }
        // }
        Ok(())
    }
}

impl ValueDescribe for Exception {
    fn fmt_describe(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl HasType for Exception {
    fn type_of(&self) -> Type {
        Type::new("error")
    }
}

impl Error for Exception {}
impl DefaultValueClone for Exception {}
impl ValueShow for Exception {}
impl ValueEq for Exception {}
impl ValueHash for Exception {}
impl Value for Exception {}

fn type_predicate<T: IsType + Value>(interpreter: &mut Interpreter) -> exec::Result<()> {
    let r = interpreter.stack.type_predicate::<T>(0)?;
    interpreter.stack.push(Datum::build().with_source(interpreter.current_source()).ok(r));
    Ok(())
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            stack: Default::default(),
            context: Context::default(),
            builtins: Default::default(),
            evaling_source: None,
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

    fn run_available_stackless(&mut self) -> exec::Result<()> {
        while let Some(d) = self.read_next()? {
            if self.quoting {
                self.quoting = false;
                self.stack.push(d);
            } else if let Ok(r) = d.value_ref::<Symbol>() {
                self.eval_symbol(&r, d.source().cloned())?;
            } else {
                self.stack.push(d);
            }
        }
        Ok(())
    }

    pub fn eval_run(&mut self) -> exec::Result<()> {
        self.run_available_stackless()
    }

    pub fn run_available(&mut self) -> Result<(), Exception> {
        let r = self.run_available_stackless();
        self.wrap_failure(r)
    }

    pub fn eval_code(&mut self, code: &Code, source: Option<Source>) -> exec::Result<()> {
        match code.value() {
            Instruction::Builtin(ref b) => {
                let mut b = self.builtins.lookup(b);
                self.evaling_source = source;
                b.call(self)?;
                // b.borrow().call(self, source)?;
            },
            Instruction::Definition(ref def) => {
                self.context.push_def(source, def);
                // self.run_available_stackless()?;
            },
        }
        Ok(())
    }

    pub fn current_source(&self) -> Option<Source> {
        self.evaling_source.clone()
    }

    pub fn resolve_symbol(&self, r: &Symbol) -> Option<Code> {
        self.context.resolve(r).cloned()
    }

    pub fn eval_symbol(&mut self, r: &Symbol, source: Option<Source>) -> exec::Result<()> {
        let code = self.resolve_symbol(r).ok_or_else(|| error::NotDefined(r.clone()))?;
        let res = self.eval_code(&code, source.clone());
        if let Err(e) = res {
            // Hack to show where the error occurred
            self.context.push_source(source);
            Err(e)?;
        }
        Ok(())
    }

    fn push_eval(&mut self, d: Datum) -> exec::Result<()> {
        self.context.add_code(d);
        self.run_available_stackless()
    }

}

impl Interpreter {
    fn wrap_failure<T, F: Into<exec::Failure>>(&self, e: Result<T, F>) -> Result<T, Exception> {
        e.map_err(|err| Exception::new(err.into(), (self.context.stack_sources(), self.context.stack_uplevel_sources())))
    }

    // Should be AsRef<Path>
    // This is manageable as a completely hosted function
    pub fn eval_file(&mut self, path: &str) -> Result<(), Exception> {
        use ::std::fs::File;
        use ::std::io::Read;
        let mut file = self.wrap_failure(File::open(&path).map_err(error::StdIoError::new))?;
        let mut contents = String::new();
        self.wrap_failure(file.read_to_string(&mut contents).map_err(error::StdIoError::new))?;

        let mut parser = Parser::new(contents.chars().into_iter()).with_file(path);

        while let Some(datum) = self.wrap_failure(parser.next())? {
            let r = self.push_eval(datum);
            self.wrap_failure(r)?;
        }
        Ok(())
    }
}

