
use std::rc::Rc;
use std::collections::HashMap;
use std::borrow::Borrow;

use crate::base::*;

/// Clone-on-write definition environment for list definitions.
#[derive(Default, Clone)]
pub struct DefSet(Rc<HashMap<String, Val>>);
impl Value for DefSet {}
impl DefSet {
    /// Add a definition.
    pub fn insert(&mut self, key: String, val: impl Into<Val>) {
        Rc::make_mut(&mut self.0).insert(key, val.into());
    }
    /// Remove a definition by name.
    pub fn remove(&mut self, key: impl AsRef<str>) -> Option<Val> {
        Rc::make_mut(&mut self.0).remove(key.as_ref())
    }
    /// See whether a definition by the given name exists.
    pub fn contains(&self, key: impl AsRef<str>) -> bool {
        self.0.contains_key(key.as_ref())
    }
    /// Look for a definition by name.
    pub fn get(&self, key: impl AsRef<str>) -> Option<&Val> {
        self.0.get(key.as_ref())
    }
    /// An iterator over the contained definition names.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(|k| k.borrow())
    }
    /// An iterator over the contained definition name/body pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Val)> {
        self.0.iter().map(|(k, v)| (k.borrow(), v))
    }
    /// Whether there are no entries.
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    /// How many entries there are.
    pub fn len(&self) -> usize { self.0.len() }

    /// Retain definitions based on the given criterion.
    pub fn filter<F: Fn(&str, &Val) -> bool>(&mut self, f: F) {
        Rc::make_mut(&mut self.0).retain(|k, v| f(k.as_ref(), v));
    }

    fn merge_with(&mut self, thee: &DefSet, overwrite: bool) {
        if thee.is_empty() { return; }
        if self.is_empty() {
            *Rc::make_mut(&mut self.0) = (*thee.0).clone();
            return;
        }
        for (k, v) in thee.iter() {
            if overwrite || !self.contains(&k) {
                self.insert(k.into(), v.clone());
            }
        }
    }

    /// Take everything from `thee` and put it in `self`.
    /// This will overwrite existing values in `self` - see also [prepend].
    pub fn append(&mut self, thee: &DefSet) {
        self.merge_with(thee, true);
    }
    /// Take everything from `thee` and put it in `self`,
    /// unless `self` already contains an entry with the same name.
    /// See also [append].
    pub fn prepend(&mut self, thee: &DefSet) {
        self.merge_with(thee, false);
    }

    /// Find (or create an empty) DefSet in meta for the value
    /// and do a function on it.
    /// Use this to e.g.
    /// add a closure environment for [List] values that will be evaluated later.
    pub fn upsert_val(v: &mut Val, f: impl FnOnce(&mut DefSet)) {
        let meta = v.meta_mut();
        let mut defs = DefSet::default();
        if !meta.contains::<DefSet>() {
            f(&mut defs);
            meta.push(defs);
        } else {
            'find_defs: for ds in meta.iter_mut() {
                if ds.try_downcast_swap::<DefSet>(&mut defs) {
                    f(&mut defs);
                    ds.try_downcast_swap(&mut defs);
                    break 'find_defs;
                }
            }
        }
    }

}

