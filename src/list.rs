
use crate::base::*;

#[derive(Debug, Clone)]
pub struct List<T> {
    data: Vec<T>,
}

impl<T> From<Vec<T>> for List<T> {
    fn from(mut data: Vec<T>) -> List<T> {
        data.reverse();
        List { data }
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut data = Vec::from_iter(iter);
        data.reverse();
        List { data }
    }

}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &List<T>) -> bool {
        if self.len() != other.len() { return false; }
        for (a, b) in self.data.iter().zip(other.data.iter()) {
            if a != b { return false; }
        }
        true
    }
}

impl<T: Eq> Eq for List<T> {}

impl<T> Default for List<T> {
    fn default() -> Self { List { data: vec![] } }
}

impl ImplValue for List<Val> { }

impl<T> List<T> {
    pub fn len(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.len() == 0 }
    pub fn pop(&mut self) -> Option<T> { self.data.pop() }
    pub fn push(&mut self, v: T) { self.data.push(v); }
}

