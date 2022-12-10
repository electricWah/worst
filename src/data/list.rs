
//! A [Vec] of [Val], basically.

use crate::base::*;

impl Value for List {}

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

    /// Remove and return the value at index
    fn remove(&mut self, i: usize) -> Option<Val> {
        if i < self.data.len() {
            Some(self.data.remove(self.data.len() - 1 - i))
        } else { None }
    }

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

    /// Check if this contains a `T`.
    pub fn contains<T: Value>(&self) -> bool {
        self.iter().any(|v| v.is::<T>())
    }

    // /// Find the first `T` and get a reference to its value.
    // pub fn first_ref_val<T: Value>(&self) -> Option<&Val> {
    //     self.iter().find(|v| v.is::<T>())
    // }

    /// Find the first `T`.
    pub fn first_ref<T: Value>(&self) -> Option<&T> {
        self.iter().find_map(|v| v.downcast_ref::<T>())
    }

    /// Find and remove the first `T`.
    pub fn take_first<T: Value>(&mut self) -> Option<ValOf<T>> {
        for i in 0 .. self.len() {
            if self.get(i).unwrap().is::<T>() {
                return self.remove(i).unwrap().try_downcast::<T>().ok();
            }
        }
        None
    }

}
