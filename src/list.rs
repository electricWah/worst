
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

impl<T> Iterator for List<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> { self.pop() }
}

impl List<Val> {
    pub fn from_vals<T: Into<Val>, I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut data = Vec::from_iter(iter.into_iter().map(|v| v.into()));
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
    pub fn get(&self, i: usize) -> Option<&T> {
        self.data.get(self.data.len() - 1 - i)
    }
    pub fn is_empty(&self) -> bool { self.data.len() == 0 }
    pub fn pop(&mut self) -> Option<T> { self.data.pop() }
    pub fn push(&mut self, v: T) { self.data.push(v); }
    pub fn top(&self) -> Option<&T> {
        if self.is_empty() { None } else { Some(&self.data[self.data.len() - 1]) }
    }

    pub fn from_pairs<K: Into<T>, V: Into<T>>(src: impl Iterator<Item=(K, V)>) -> Self {
        let mut data = vec![];
        for (k, v) in src {
            data.push(v.into());
            data.push(k.into());
        }
        List { data }
    }
    pub fn reverse(&mut self) { self.data.reverse() }
}

impl List<Val> {
    pub fn pairs_find_key(&self, v: impl Value) -> Option<&Val> {
        for i in 0 .. self.len()/2 {
            if let Some(k) = self.get(i * 2) {
                // dbg!(i, &k);
                if v.equal(&k) {
                    return self.get(i * 2 + 1);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs() {
        let thing =
            List::from_pairs(vec![
                ("test".to_string().into(), 5.into()),
                (Val::from("beans".to_string()), Val::from(7)),
            ].into_iter());
        assert_eq!(Some(&5.into()), thing.pairs_find_key("test".to_string()));
        assert_eq!(Some(&7.into()), thing.pairs_find_key("beans".to_string()));
        assert_eq!(None, thing.pairs_find_key(123));
    }
}

