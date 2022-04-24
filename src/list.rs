
// use std::cell::{Ref, RefCell}; // no refcell until push/pop needs more fast
use crate::base::*;

#[derive(Debug, Clone)]
pub struct List {
    data: Vec<Val>,
}

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

impl PartialEq for List {
    fn eq(&self, other: &List) -> bool {
        if self.len() != other.len() { return false; }
        for (a, b) in self.data.iter().zip(other.data.iter()) {
            if a != b { return false; }
        }
        true
    }
}

impl Eq for List {}

impl Default for List {
    fn default() -> Self { List::from(vec![]) }
}

impl ImplValue for List { }

impl List {
    pub fn len(&self) -> usize { self.data.len() }
    pub fn get(&self, i: usize) -> Option<&Val> {
        if i < self.data.len() {
            Some(&self.data[self.data.len() - 1 - i])
        } else { None }
    }
    pub fn is_empty(&self) -> bool { self.data.len() == 0 }

    pub fn pop(&mut self) -> Option<Val> {
        self.data.pop()
    }
    pub fn push(&mut self, v: Val) {
        self.data.push(v);
    }
    pub fn top(&self) -> Option<&Val> { self.get(0) }

    pub fn from_pairs<K: Into<Val>, V: Into<Val>>(src: impl Iterator<Item=(K, V)>) -> Self {
        let mut data = vec![];
        for (k, v) in src {
            data.push(v.into());
            data.push(k.into());
        }
        List { data }
    }
    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    // pub fn pairs_find_key(&self, v: impl Value) -> Option<&Val> {
    //     for i in 0 .. self.len()/2 {
    //         if let Some(k) = self.get(i * 2) {
    //             // dbg!(i, &k);
    //             if v == k {
    //                 return self.get(i * 2 + 1);
    //             }
    //         }
    //     }
    //     None
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn pairs() {
//         let thing =
//             List::from_pairs(vec![
//                 ("test".to_string().into(), 5.into()),
//                 (Val::from("beans".to_string()), Val::from(7)),
//             ].into_iter());
//         assert_eq!(Some(&5.into()), thing.pairs_find_key("test".to_string()));
//         assert_eq!(Some(&7.into()), thing.pairs_find_key("beans".to_string()));
//         assert_eq!(None, thing.pairs_find_key(123));
//     }
// }

