
pub mod code;
mod context;
pub mod command;
pub mod exec;
mod stack;

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
    pub reader: Reader,
    pub stack: Stack,
    pub context: Context,
    gensym: usize,
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

impl Interpreter {
    pub fn new(reader: Reader) -> Self {
        let parser = Parser::new(Source::new(), &reader);
        Interpreter {
            reader,
            stack: Default::default(),
            context: Context::default().with_parser(parser),
            gensym: Default::default(),
            quoting: false,
        }
    }

    pub fn quoting(&self) -> bool {
        self.quoting
    }

    pub fn set_quoting(&mut self, q: bool) {
        self.quoting = q;
    }

    pub fn gensym(&mut self) -> usize {
        self.gensym += 1;
        self.gensym
    }

    pub fn quote_next(&mut self) {
        self.quoting = true;
    }

    pub fn reader_mut(&mut self) -> &mut Reader {
        &mut self.reader
    }

    pub fn define<S: Into<Symbol>, C: Into<Code>>(&mut self, name: S, code: C) {
        self.context.define(name, code);
    }

    pub fn clear(&mut self) {
        self.context.reset();
        self.quoting = false;
    }

    pub fn unfinished(&self) -> Vec<&str> {
        match self.context.parser() {
            Some(p) => p.unfinished(),
            None => vec![],
        }
    }
}

impl Interpreter {

    pub fn read_next(&mut self) -> exec::Result<Option<Datum>> {
        self.context.next(&self.reader, self.quoting)
    }

    fn run_available_stackless(&mut self) -> exec::Result<()> {
        while let Some(d) = self.read_next()? {
            if self.quoting {
                self.quoting = false;
                self.stack.push(d);
            } else {
                self.eval_datum(d)?;
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
            Instruction::Command(ref cmd) => {
                cmd.run(self, source)?;
            },
            Instruction::Definition(ref def) => {
                self.context.push_def(source, def);
            },
        }
        Ok(())
    }

    pub fn resolve_symbol(&self, r: &Symbol) -> Option<Code> {
        self.context.resolve(r).cloned()
    }

    pub fn eval_symbol(&mut self, r: &Symbol, source: Option<Source>) -> exec::Result<()> {
        debug!("Eval {:?}", r.as_ref());
        let code = self.resolve_symbol(r).ok_or(error::NotDefined())?;
        let res = self.eval_code(&code, source.clone());
        if let Err(e) = res {
            // Hack to show where the error occurred
            self.context.push_source(source);
            Err(e)?;
        }
        Ok(())
    }

    pub fn eval_datum(&mut self, d: Datum) -> exec::Result<()> {
        debug!("Interpret {}", d.dump());
        trace!("Stack: {}", self.stack.show());
        if let Ok(r) = d.value_ref::<Symbol>() {
            return self.eval_symbol(&r, d.source().cloned());
        }
        self.stack.push(d);
        Ok(())
    }

}

impl Interpreter {
    fn wrap_failure<T>(&self, e: exec::Result<T>) -> Result<T, Exception> {
        e.map_err(|err| Exception::new(err, (self.context.stack_sources(), self.context.stack_uplevel_sources())))
    }

    pub fn push_input(&mut self, input: &str) {
        let mut parser = {
            match self.context.take_parser() {
                Some(p) => p,
                None => {
                    let src = self.context.source().cloned().unwrap_or(Source::new());
                    Parser::new(src, &self.reader)
                },
            }
        };

        parser.push_input(&mut input.chars());

        self.context.set_parser(parser);
    }

    // Should be AsRef<Path>
    // This is manageable as a completely hosted function
    pub fn load_file(&mut self, path: &str) -> exec::Result<()> {
        info!("Loading file {}", path);
        use ::std::fs::File;
        use ::std::io::Read;
        let mut file = File::open(&path).map_err(error::StdIoError::new)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(error::StdIoError::new)?;

        let start_pos = Source::new().with_file(path.to_string());
        let mut parser = Parser::new(start_pos, &self.reader);
        parser.push_input(&mut contents.chars());
        parser.set_eof(true);
        self.context.set_parser(parser);
        Ok(())
    }

}

