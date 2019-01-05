
use std::collections::HashMap;
use std::fmt;
use crate::data::*;
use crate::data::error::*;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct HashTable(pub HashMap<BoxValue, Datum>);

impl StaticType for HashTable {
    fn static_type() -> Type {
        Type::new("hash-table")
    }
}

impl ValueShow for HashTable {
    fn fmt_show(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<hash-table({})>", self.0.len())
    }
}

impl DefaultValueEq for HashTable {}
impl DefaultValueClone for HashTable {}
impl ValueDebugDescribe for HashTable {}
impl ValueHash for HashTable {}
impl Value for HashTable {}

impl HashTable {
    pub fn size(&self) -> usize {
        self.0.len()
    }
    pub fn set(&mut self, k: Datum, v: Datum) {
        self.0.insert(k.into_boxed(), v);
    }
    pub fn exists(&self, k: &Datum) -> bool {
        self.0.contains_key(k.as_boxed())
    }
    pub fn take(&mut self, k: &Datum) -> Option<Datum> {
        self.0.remove(k.as_boxed())
    }
    pub fn get(&self, k: &Datum) -> Option<&Datum> {
        self.0.get(k.as_boxed())
    }
    pub fn keys(&self) -> impl Iterator<Item=&BoxValue> {
        self.0.keys()
    }
    pub fn random_key(&self) -> Option<Datum> {
        self.0.keys().next().cloned().map(Datum::from_boxed)
    }
    pub fn take_random_pair(&mut self) -> Option<(Datum, Datum)> {
        // TODO don't allocate key again
        let k = self.0.keys().next().cloned();
        match k {
            Some(k) =>
                self.0.remove_entry(&k)
                    .map(|(k, v)| (Datum::from_boxed(k), v)),
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct HashTableEmpty();
impl Error for HashTableEmpty {}

impl fmt::Display for HashTableEmpty {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Hashtable empty")
    }
}

#[derive(Debug)]
pub struct NoSuchKey();
impl Error for NoSuchKey {}

impl fmt::Display for NoSuchKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Key does not exist")
    }
}

