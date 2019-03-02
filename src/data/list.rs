
use std::fmt;
pub use std::iter::FromIterator;
use std::collections::VecDeque;
use crate::data::value::*;
use crate::data::datum::*;
use crate::data::types::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct List(VecDeque<Datum>);

impl Into<Vec<Datum>> for List {
    fn into(self) -> Vec<Datum> {
        self.0.into()
    }
}

impl Into<VecDeque<Datum>> for List {
    fn into(self) -> VecDeque<Datum> {
        self.0
    }
}

impl FromIterator<Datum> for List {
    fn from_iter<I: IntoIterator<Item=Datum>>(iter: I) -> Self {
        List(iter.into_iter().collect())
    }
}

impl<'a> FromIterator<&'a Datum> for List {
    fn from_iter<I: IntoIterator<Item=&'a Datum>>(iter: I) -> Self {
        List(iter.into_iter().map(Clone::clone).collect())
    }
}

impl From<Vec<Datum>> for List {
    fn from(l: Vec<Datum>) -> List {
        List(l.into())
    }
}

impl<T: Value> From<Vec<T>> for List {
    fn from(l: Vec<T>) -> List {
        List(l.into_iter().map(Datum::new).collect())
    }
}

impl List {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j)
    }
    pub fn push_head(&mut self, d: Datum) {
        self.0.push_front(d)
    }
    pub fn push_tail(&mut self, d: Datum) {
        self.0.push_back(d)
    }
    pub fn pop_head(&mut self) -> Option<Datum> {
        self.0.pop_front()
    }
    pub fn pop_tail(&mut self) -> Option<Datum> {
        self.0.pop_back()
    }
    pub fn append(&mut self, mut other: List) {
        self.0.append(&mut other.0)
    }
}

impl List {
    pub fn fmt_show_list<'a, V: Iterator<Item=&'a Datum>>
                        (list: V, fmt: &mut fmt::Formatter)
                        -> fmt::Result {
        write!(fmt, "[")?;
        for (i, v) in list.enumerate() {
            if i > 0 {
                write!(fmt, " ")?;
            }
            v.fmt_show(fmt)?;
        }
        write!(fmt, "]")
    }
    pub fn fmt_describe_list<'a, V: Iterator<Item=&'a Datum>>
                            (list: V, fmt: &mut fmt::Formatter)
                            -> fmt::Result {
        write!(fmt, "[")?;
        for (i, v) in list.enumerate() {
            if i > 0 {
                write!(fmt, " ")?;
            }
            v.fmt_describe(fmt)?;
        }
        write!(fmt, "]")
    }

}

impl StaticType for List {
    fn static_type() -> Type {
        Type::new("list")
    }
}

impl ValueDescribe for List {
    fn fmt_describe(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        List::fmt_describe_list(self.0.iter(), fmt)
    }
}

impl ValueShow for List {
    fn fmt_show(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        List::fmt_show_list(self.0.iter(), fmt)
    }
}

impl ValueHash for List {
    fn can_hash_value(&self) -> bool {
        self.0.iter().all(|v| v.can_hash_value())
    }
    fn hash_value(&self, state: &mut ValueHasher) {
        for v in self.0.iter() {
            v.hash_value(state)
        }
    }
}

impl DefaultValueEq for List {}
impl DefaultValueHash for List {}
impl DefaultValueClone for List {}
impl Value for List {}

