
//! A [Vec] of [Val], basically.

use std::fmt;
use crate::base::*;

/// A list of [Val] values. It is itself a [Value].
/// This is the primary container type in Worst.
/// It's a little like a Lisp list.
#[derive(Clone, Default)]
pub struct List {
    // Rc<> this if cloning takes a long time
    data: Vec<Val>,
}
value!(List: dyn fmt::Display);

impl From<Vec<Val>> for List {
    fn from(mut data: Vec<Val>) -> List {
        data.reverse();
        List { data }
    }
}

impl<T: Value> FromIterator<T> for List {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        List::from(Vec::from_iter(iter.into_iter().map(Into::into)))
    }
}

impl Iterator for List {
    type Item = Val;
    fn next(&mut self) -> Option<Val> { self.pop() }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        let mut space = false;
        for v in self.iter() {
            if space { write!(f, " ")?; }
            space = true;
            if let Some(d) = v.as_trait_ref::<dyn fmt::Display>() {
                write!(f, "{}", d)?;
            } else {
                write!(f, "<value>")?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl List {
    /// Get the number of values in this list.
    pub fn len(&self) -> usize { self.data.len() }
    /// Get a value by index, if the index is in range. 0 is at the front.
    pub fn get(&self, i: usize) -> Option<&Val> {
        if i < self.data.len() {
            Some(&self.data[self.data.len() - 1 - i])
        } else { None }
    }
    /// Is this list devoid of contents?
    pub fn is_empty(&self) -> bool { self.data.is_empty() }

    /// Iterate this list from front to back :)
    pub fn iter(&self) -> impl Iterator<Item=&Val> { self.data.iter().rev() }

    /// Iterate this list from front to back - mutable edition!
    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut Val> {
        self.data.iter_mut().rev()
    }

    /// Take the front value from this list, if it isn't empty.
    pub fn pop(&mut self) -> Option<Val> {
        self.data.pop()
    }
    /// Put just one value at the front of this list.
    pub fn push(&mut self, v: impl Into<Val>) {
        self.data.push(v.into());
    }
    /// Put the contents of an entire list in front of this list.
    pub fn prepend(&mut self, mut other: List) {
        self.data.append(&mut other.data);
    }
    /// Get the value at the front of this list, if it isn't empty.
    pub fn top(&self) -> Option<&Val> { self.get(0) }

    /// Build a list from the given iterator,
    /// shaped like `(key value key value ...)`
    pub fn from_pairs<K: Into<Val>, V: Into<Val>>(src: impl Iterator<Item=(K, V)>) -> Self {
        let mut data = vec![];
        for (k, v) in src {
            data.push(v.into());
            data.push(k.into());
        }
        List { data }
    }
    /// Reverse the list in-place.
    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    /// Pop the first n elements from the list into the returned list.
    /// Out-of-range values result in returning the list unchanged and
    /// setting `self` to an empty list.
    pub fn pop_n(&mut self, count: usize) -> List {
        if count > self.len() {
            let mut l = List::default();
            std::mem::swap(self, &mut l);
            l
        } else {
            List { data: self.data.split_off(self.len() - count) }
        }
    }

    /// Clone the first n elements into a new list.
    /// Out-of-range values are clamped to the length of the list.
    pub fn top_n(&self, count: usize) -> List {
        let len = self.len();
        if count > len {
            self.clone()
        } else {
            let data = self.data[len - count ..].to_vec();
            List { data }
        }
    }
}

