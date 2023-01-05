
//! Prehashed hashtable for Worst values

use std::hash::{ Hash, Hasher };
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use crate::base::*;

/// A computed hash of a Value.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ValueHash(u64);
impl Value for ValueHash {}

impl ValueHash {
    /// Compute the hash of the given value.
    pub fn hash_value<T: Hash>(val: impl AsRef<T>) -> Self {
        let mut hasher = DefaultHasher::new();
        val.as_ref().hash(&mut hasher);
        ValueHash(hasher.finish())
    }
}

// doesn't even have to be a HashMap I guess!
/// A HashMap where the hashes are precomputed.
#[derive(Default, Clone)]
pub struct I64Table {
    data: HashMap<i64, Val>,
}

impl Value for I64Table {}

impl I64Table {
    /// Insert a key/value pair. The key should have the given hash.
    pub fn insert(&mut self, k: i64, v: Val) {
        self.data.insert(k, v);
    }

    /// Find the value for the given hash.
    pub fn get(&self, hash: &ValueHash) -> Option<&Val> {
        self.data.get(hash)
    }

    /// Find all the keys.
    pub fn keys(&self) -> impl Iterator<Item=&Val> {
        self.data.keys()
    }
}

