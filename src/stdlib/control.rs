
use data::*;
use parser::Source;
use interpreter::command::*;
use interpreter::code::*;
use interpreter::exec;
use interpreter::Interpreter;
use stdlib::enumcommand::*;

pub fn install(interpreter: &mut Interpreter) {
    Control::install(interpreter);
}

#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Control {
    Uplevel,
    Quote,
    ListIntoDefinition,
    GetDefinition,
    TakeDefinition,
    ResolveDefinition,
    AddDefinition,
    EvalDefinition,
    IsDefined,
    IsDefinition,
    ListDefinedNames,
    GetDefineMeta,
    SetDefineMeta,
    TakeDefineMeta,
    Call,
    CallWhen,
    InterpreterIsRootContext,
    InterpreterSetContextName,
    InterpreterContextName,
    UplevelInNamedContext,
    InterpreterReadFile,
    // InterpreterReadChar,
    // InterpreterReadEof,
    // InterpreterEvalRead,
    Abort,
    Gensym, // Keep gensym at the bottom
}

impl EnumCommand for Control {
    fn as_str(&self) -> &str {
        use self::Control::*;
        match self {
            Uplevel => "uplevel",
            Quote => "quote",
            ListIntoDefinition => "list->definition",
            GetDefinition => "get-definition",
            TakeDefinition => "take-definition",
            ResolveDefinition => "resolve-definition",
            AddDefinition => "add-definition",
            EvalDefinition => "eval-definition",
            IsDefined => "defined?",
            IsDefinition => "definition?",
            ListDefinedNames => "defined-names",
            GetDefineMeta => "definition-get-meta",
            SetDefineMeta => "definition-set-meta",
            TakeDefineMeta => "definition-take-meta",
            Call => "call",
            CallWhen => "call-when",
            InterpreterIsRootContext => "interpreter-root-context?",
            InterpreterSetContextName => "interpreter-set-context-name",
            InterpreterContextName => "interpreter-context-name",
            UplevelInNamedContext => "uplevel-in-named-context",
            InterpreterReadFile => "interpreter-read-file",
            // InterpreterReadChar => "interpreter-read-char",
            // InterpreterReadEof => "interpreter-read-eof",
            // InterpreterEvalRead => "interpreter-eval-read",
            Abort => "abort",
            Gensym => "gensym",
        }
    }
    fn last() -> Self { Control::Gensym }
    fn from_usize(s: usize) -> Self { unsafe { ::std::mem::transmute(s) } }
}

impl Command for Control {
    fn run(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
        debug!("Control: {:?}", self);
        use self::Control::*;
        match self {
            &Uplevel => {
                // interpreter.stack.expect(&[DatumType::Symbol.into()])?;
                interpreter.context.uplevel(source.clone())?;
                let (name, source) = interpreter.stack.pop_source::<Symbol>()?;
                return interpreter.eval_symbol(&name, source);
            },
            &Quote => {
                interpreter.quote_next();
            },
            &ListIntoDefinition => {
                let code = interpreter.stack.pop::<List>()?.into();
                let def = Code::from(Definition::new(code).with_source(source));
                interpreter.stack.push(Datum::new(def));
            },
            &GetDefinition => {
                let name = interpreter.stack.pop::<Symbol>()?;
                match interpreter.context.get_definition(&name) {
                    Some(def) => {
                        interpreter.stack.push(Datum::new(def));
                    },
                    None => Err(error::NotDefined())?,
                }
            },
            &TakeDefinition => {
                let name = interpreter.stack.pop::<Symbol>()?;
                match interpreter.context.undefine(&name) {
                    Some(def) => {
                        interpreter.stack.push(Datum::new(def));
                    },
                    None => Err(error::NotDefined())?,
                }
            },
            &ResolveDefinition => {
                let name = interpreter.stack.pop::<Symbol>()?;
                match interpreter.context.resolve(&name) {
                    Some(def) => {
                        interpreter.stack.push(Datum::new(def.clone()));
                    },
                    None => Err(error::NotDefined())?,
                }
            },
            &AddDefinition => {
                let name = interpreter.stack.pop::<Symbol>()?;
                let def = interpreter.stack.pop::<Code>()?;
                interpreter.context.define(name, def);
            },
            &EvalDefinition => {
                let (code, source) = interpreter.stack.pop_source::<Code>()?;
                interpreter.eval_code(&code, source)?;
                // interpreter.stack.push(code);
            },
            &IsDefined => {
                let r = {
                    let name = interpreter.stack.ref_at::<Symbol>(0)?;
                    interpreter.context.is_defined(name)
                };
                interpreter.stack.push(Datum::new(r));
            },
            &IsDefinition => {
                let r = interpreter.stack.type_predicate::<Code>(0)?;
                interpreter.stack.push(Datum::new(r));
            },
            &ListDefinedNames => {
                // TODO source
                let names: Vec<Datum> = interpreter.context.current_defines()
                    .map(Clone::clone)
                    .map(|s| Datum::build().symbol(s))
                    .collect();
                interpreter.stack.push(Datum::new::<List>(names.into()));
            },
            &GetDefineMeta => {
                let name = interpreter.stack.pop::<Symbol>()?;
                let def = interpreter.context.resolve(&name);
                match def {
                    None => interpreter.stack.push(Datum::new(false)),
                    Some(d) => {
                        match d.meta().cloned() {
                            None => interpreter.stack.push(Datum::new(false)),
                            Some(m) => interpreter.stack.push(m),
                        }
                    },
                }
            },
            &SetDefineMeta => {
                let meta = interpreter.stack.pop_datum()?;
                let name = interpreter.stack.pop::<Symbol>()?;
                let mut def = interpreter.context.undefine(&name);
                match def {
                    None => Err(exec::Exception::from(error::NotDefined()))?,
                    Some(mut d) => {
                        d.set_meta(meta);
                        interpreter.context.define(name, d);
                    },
                }
            },
            &TakeDefineMeta => {
                let name = interpreter.stack.pop::<Symbol>()?;
                let mut def = interpreter.context.undefine(&name);
                match def {
                    None => Err(exec::Exception::from(error::NotDefined()))?,
                    Some(mut d) => {
                        match d.take_meta() {
                            None => interpreter.stack.push(Datum::new(false)),
                            Some(m) => interpreter.stack.push(m),
                        }
                        interpreter.context.define(name, d);
                    },
                }
            },
            &Call => {
                let (name, source) = interpreter.stack.pop_source::<Symbol>()?;
                return interpreter.eval_symbol(&name, source);
            },
            &CallWhen => {
                let (name, source) = interpreter.stack.pop_source::<Symbol>()?;
                let whether = interpreter.stack.pop::<bool>()?;
                if whether {
                    return interpreter.eval_symbol(&name, source);
                }
            },
            &InterpreterIsRootContext => {
                let r = interpreter.context.is_root();
                interpreter.stack.push(Datum::new(r));
            },
            &InterpreterSetContextName => {
                let name = interpreter.stack.pop::<Symbol>()?;
                interpreter.context.set_name(Some(name.to_string()));
            },
            &InterpreterContextName => {
                let name = interpreter.context.name().map(Symbol::from);
                match name {
                    Some(n) => interpreter.stack.push(Datum::new(n)),
                    None => interpreter.stack.push(Datum::new(false)),
                }
            },
            &UplevelInNamedContext => {
                let name = interpreter.stack.pop::<Symbol>()?;
                let sym = interpreter.stack.pop::<Symbol>()?;
                while interpreter.context.name() != Some(name.as_ref()) {
                    interpreter.context.uplevel(None)?;
                }
                interpreter.eval_symbol(&sym, source)?;
            },
            &InterpreterReadFile => {
                let file = interpreter.stack.pop::<String>()?;
                interpreter.load_file(&file)?;
            },
            // &InterpreterReadChar => {
            //     let c = interpreter.stack.pop::<char>()?;
            //     interpreter.parse_run(c)?;
            // },
            // &InterpreterReadEof => {
            //     interpreter.parse_run(None)?;
            // },
            // &InterpreterEvalRead => {
            //     interpreter.eval_run()?;
            // },
            &Abort => {
                Err(error::Abort())?;
            }
            &Gensym => {
                let (name, orig_source) = interpreter.stack.pop_source::<Symbol>()?;
                let sym = {
                    let id = interpreter.gensym();
                    let mut ss = name.to_string();
                    ss.push_str(format!("-{}", id).as_str());
                    ss
                };
                interpreter.stack.push(Datum::build().with_source(orig_source).ok(name));
                interpreter.stack.push(Datum::build().with_source(source).ok(sym));
            },
        }
        Ok(())
    }
}

