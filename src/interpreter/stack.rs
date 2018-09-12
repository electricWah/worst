
use std::fmt;
use parser::*;
use data::*;
use data::error::*;
use interpreter::exec;

#[derive(Default, Debug)]
pub struct Stack {
    transaction: Option<Vec<StackOp>>,
    stack: Vec<Datum>,
}

#[derive(Debug)]
enum StackOp {
    Push(Datum),
    Pop,
}

pub struct TransactionHandle();

pub struct StackShow<'a>(&'a Stack);
pub struct StackDescribe<'a>(&'a Stack);

impl Stack {
    pub fn new() -> Self {
        Stack::default()
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

    fn pop_stack(&mut self) -> Option<Datum> {
        match self.stack.pop() {
            Some(d) => {
                match &mut self.transaction {
                    &mut Some(ref mut ops) => {
                        ops.push(StackOp::Push(d.clone()));
                    },
                    &mut None => {},
                }
                Some(d)
            },
            None => None,
        }
    }

    pub fn push(&mut self, d: Datum) {
        match &mut self.transaction {
            &mut Some(ref mut ops) => {
                ops.push(StackOp::Pop);
            },
            &mut None => {},
        }
        self.stack.push(d);
    }

    pub fn pop_datum(&mut self) -> Result<Datum, StackEmpty> {
        self.pop_stack().ok_or(StackEmpty())
    }
    pub fn pop_datum_source(&mut self) -> Result<Datum, StackEmpty> {
        self.pop_stack().ok_or(StackEmpty())
    }

    pub fn insert(&mut self, d: Datum, idx: usize) -> Result<(), StackEmpty> {
        let len = self.stack.len();
        if idx >= len { Err(StackEmpty())?; }
        self.stack.insert(len - idx - 1, d);
        Ok(())
    }

    pub fn remove(&mut self, idx: usize) -> Result<Datum, StackEmpty> {
        let len = self.stack.len();
        if idx >= len { Err(StackEmpty())?; }
        let d = self.stack.remove(len - idx - 1);
        Ok(d)
    }

    pub fn top_mut_datum(&mut self) -> Result<&mut Datum, StackEmpty> {
        let len = self.stack.len();
        if len == 0 {
            Err(StackEmpty())
        } else {
            Ok(&mut self.stack[len - 1])
        }
    }
    pub fn ref_datum(&self, idx: usize) -> Result<&Datum, StackEmpty> {
        let len = self.stack.len();
        if idx < len {
            Ok(&self.stack[len - idx - 1])
        } else {
            Err(StackEmpty())
        }
    }

    pub fn pop<T: IsType + Value + Sized>(&mut self) -> exec::Result<T> {
        let datum = self.pop_datum()?;
        datum.into_value::<T>().map_err(|t| WrongType(T::get_type(), t).into())
    }

    pub fn pop_source<T: IsType + Value + Sized>(&mut self) -> exec::Result<(T, Option<Source>)> {
        let datum = self.pop_datum()?;
        datum.into_value_source::<T>().map_err(|t| WrongType(T::get_type(), t).into())
    }

    pub fn ref_at<T: IsType + Value>(&self, idx: usize) -> exec::Result<&T> {
        let datum = self.ref_datum(idx)?;
        datum.value_ref::<T>().map_err(|t| WrongType(T::get_type(), t).into())
    }

    pub fn type_predicate<T: IsType + Value>(&self, idx: usize) -> exec::Result<bool> {
        let datum = self.ref_datum(idx)?;
        Ok(datum.value_ref::<T>().is_ok())
    }

    /// If you use this, make sure any errors are thrown before mutating the value.
    pub fn top_mut<T: IsType + Value>(&mut self) -> exec::Result<&mut T> {
        let m = self.top_mut_datum()?;
        m.value_mut::<T>().map_err(|t| WrongType(T::get_type(), t).into())
    }

    pub fn transaction(&mut self) -> TransactionHandle {
        self.transaction = Some(vec![]);
        TransactionHandle()
    }

    pub fn commit(&mut self, _t: TransactionHandle) {
        self.transaction = None;
    }

    pub fn rollback(&mut self, _t: TransactionHandle) {
        match self.transaction.take() {
            None => {},
            Some(mut ops) => {
                while let Some(op) = ops.pop() {
                    match op {
                        StackOp::Push(d) => self.stack.push(d),
                        StackOp::Pop => { self.stack.pop(); },
                    }
                }
            },
        }
    }
    pub fn describe<'a>(&'a self) -> StackDescribe<'a> {
        StackDescribe(&self)
    }
    pub fn show<'a>(&'a self) -> StackShow<'a> {
        StackShow(&self)
    }
}

impl<'a> fmt::Display for StackShow<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        List::fmt_show_list(self.0.stack.iter(), fmt)
    }
}

impl<'a> fmt::Display for StackDescribe<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for (i, d) in self.0.stack.iter().enumerate() {
            write!(fmt, "#{} ", i)?;
            fmt::Display::fmt(&d.dump(), fmt)?;
        }
        Ok(())
    }
}

