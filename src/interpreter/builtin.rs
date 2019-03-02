
use std::collections::HashMap;
use crate::data::*;
use crate::interpreter::Interpreter;
use crate::interpreter::exec;

/// A dictionary of BuiltinRef -> actual code
#[derive(Default)]
pub struct BuiltinLookup(HashMap<Symbol, Box<BuiltinFn>>);

impl BuiltinLookup {
    pub fn add<B: BuiltinFn + 'static>(&mut self, name: Symbol, builtin: B) {
        self.0.insert(name, Box::new(builtin));
    }

    pub fn lookup(&self, s: &Symbol) -> Option<Box<BuiltinFn>> {
        match self.0.get(s) {
            Some(b) => Some(b.builtin_clone()),
            None => None,
        }
    }
}

pub trait BuiltinFnArg: Sized {
    fn extract(i: &mut Interpreter) -> exec::Result<Self>;
}

impl<T: IsType + Value> BuiltinFnArg for T {
    fn extract(i: &mut Interpreter) -> exec::Result<Self> {
        i.stack.pop::<T>()
    }
}

impl BuiltinFnArg for Datum {
    fn extract(i: &mut Interpreter) -> exec::Result<Self> {
        Ok(i.stack.pop_datum()?)
    }
}

pub trait BuiltinFnArgs: Sized {
    fn extract(interpreter: &mut Interpreter) -> exec::Result<Self>;
}

// impl<'a> BuiltinFnArgs<'a> for &'a mut Interpreter {
//     fn extract(i: &'a mut Interpreter) -> exec::Result<Self> {
//         Ok(i)
//     }
// }

impl BuiltinFnArgs for () {
    fn extract(_interpreter: &mut Interpreter) -> exec::Result<Self> {
        Ok(())
    }
}

impl<A: BuiltinFnArg> BuiltinFnArgs for A {
    fn extract(interpreter: &mut Interpreter) -> exec::Result<Self> {
        A::extract(interpreter)
    }
}

impl<A: BuiltinFnArg, B: BuiltinFnArg> BuiltinFnArgs for (A, B) {
    fn extract(interpreter: &mut Interpreter) -> exec::Result<Self> {
        let a = A::extract(interpreter)?;
        let b = B::extract(interpreter)?;
        Ok((a, b))
    }
}

impl<A: BuiltinFnArg, B: BuiltinFnArg, C: BuiltinFnArg> BuiltinFnArgs for (A, B, C) {
    fn extract(interpreter: &mut Interpreter) -> exec::Result<Self> {
        let a = A::extract(interpreter)?;
        let b = B::extract(interpreter)?;
        let c = C::extract(interpreter)?;
        Ok((a, b, c))
    }
}

pub trait BuiltinFnRet {
    fn into_datum(self) -> Datum;
}

impl<T: Value> BuiltinFnRet for T {
    fn into_datum(self) -> Datum {
        Datum::new(self)
    }
}

impl BuiltinFnRet for Datum {
    fn into_datum(self) -> Datum {
        self
    }
}

pub trait BuiltinFnRets {
    fn into_datums(self) -> Vec<Datum>;
}

impl BuiltinFnRets for () {
    fn into_datums(self) -> Vec<Datum> {
        vec![]
    }
}

impl<A: BuiltinFnRet> BuiltinFnRets for A {
    fn into_datums(self) -> Vec<Datum> {
        vec![self.into_datum()]
    }
}

impl<A: BuiltinFnRet, B: BuiltinFnRet> BuiltinFnRets for (A, B) {
    fn into_datums(self) -> Vec<Datum> {
        vec![self.0.into_datum(), self.1.into_datum()]
    }
}

impl<A: BuiltinFnRet, B: BuiltinFnRet, C: BuiltinFnRet> BuiltinFnRets for (A, B, C) {
    fn into_datums(self) -> Vec<Datum> {
        vec![self.0.into_datum(), self.1.into_datum(), self.2.into_datum()]
    }
}

pub trait BuiltinFnClone {
    fn builtin_clone(&self) -> Box<BuiltinFn>;
}

impl<T: 'static + BuiltinFn + Clone> BuiltinFnClone for T {
    fn builtin_clone(&self) -> Box<BuiltinFn> {
        Box::new(self.clone())
    }
}

pub trait BuiltinFn: BuiltinFnClone {
    fn call(&mut self, interpreter: &mut Interpreter) -> exec::Result<()>;
}

// impl<T: Clone + BuiltinFn> BuiltinFn for Box<T> {
//     fn call(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
//         self.call(interpreter, source)
//     }
// }

// impl<'a, T: BuiltinFn> BuiltinFn for &'a T {
//     fn call(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
//         (*self).call(interpreter, source)
//     }
// }

// impl BuiltinFn for fn(&mut Interpreter, Option<Source>) -> exec::Result<()> {
//     fn call(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
//         self(interpreter, source)
//     }
// }

// impl<R: BuiltinFnRets, F: Clone + Fn() -> exec::Result<R>> BuiltinFn for F {
//     fn call(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
//         for r in self()?.into_datums().into_iter() {
//             interpreter.stack.push(r.into_datum());
//         }
//         Ok(())
//     }
// }

// impl<A, R, F> Into<B<A, R, F>> for F {
//     fn into(self) -> B<A, R, F> {
//         B { a: PhantomData, r: PhantomData, f: self, }
//     }
// }

// impl<A: BuiltinFnArgs, R: BuiltinFnRets, F: Clone> BuiltinFn for B<A, R, F> where F: FnMut(A) -> exec::Result<R> {
//     fn call(&mut self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
//         let args = A::extract(interpreter)?;
//         let ret = (self.f)(args)?;
//         for r in ret.into_datums().into_iter() {
//             interpreter.stack.push(r.into_datum());
//         }
//         Ok(())
//     }
// }

impl<R: BuiltinFnRets, F: 'static + Clone + FnMut(&mut Interpreter) -> exec::Result<R>> BuiltinFn for F {
    fn call(&mut self, interpreter: &mut Interpreter) -> exec::Result<()> {
        for r in self(interpreter)?.into_datums().into_iter() {
            interpreter.stack.push(r.into_datum());
        }
        Ok(())
    }
}

// impl<R: BuiltinFnRets> BuiltinFn for &FnMut(&mut Interpreter) -> exec::Result<R> {
//     fn call(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
//         for r in self(interpreter)?.into_datums().into_iter() {
//             interpreter.stack.push(r.into_datum());
//         }
//         Ok(())
//     }
// }

// impl<R: BuiltinFnRets> BuiltinFn for fn() -> exec::Result<R> {
//     fn call(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
//         for r in self()?.into_datums().into_iter() {
//             interpreter.stack.push(r.into_datum());
//         }
//         Ok(())
//     }
// }

// impl<A: BuiltinFnArg, R: BuiltinFnRets> BuiltinFn for &Fn(A) -> exec::Result<R> {
//     fn call(&self, interpreter: &mut Interpreter, source: Option<Source>) -> exec::Result<()> {
//         let a = BuiltinFnArg::extract(interpreter)?;
//         for r in self(a)?.into_datums().into_iter() {
//             interpreter.stack.push(r.into_datum());
//         }
//         Ok(())
//     }
// }

