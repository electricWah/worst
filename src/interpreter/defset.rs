
use std::rc::Rc;
use std::collections::HashMap;

use crate::base::*;

/// Kinda-clone-on-write definition environment for list definitions.
/// 
/// Keeps a "base" set of definitions and tracks updates on top of that.
/// For most programs the base set should remain fairly constant.
#[derive(Default, Clone)]
pub struct DefSet {
    base: Rc<HashMap<String, Val>>,
    updates: HashMap<String, Val>, //Option<Val>>,
}
impl Value for DefSet {}

impl DefSet {
    /// Add a definition.
    pub fn insert(&mut self, key: impl Into<String>, val: impl Into<Val>) {
        self.updates.insert(key.into(), val.into()); // Some(val.into()));
    }
    /// Remove a definition by name.
    pub fn remove(&mut self, key: impl AsRef<str>) -> Option<Val> {
        let key = key.as_ref();
        if self.base.contains_key(key) {
            todo!("base contains");
        }
        self.updates.remove(key)
        // match self.updates.insert(key.to_string(), None) {
        //     // inserted and then removed
        //     Some(None) => None,
        //     Some(old) => old,
        //     // "removed"
        //     None => self.base.get(key).cloned(),
        // }
    }
    /// See whether a definition by the given name exists.
    pub fn contains(&self, key: impl AsRef<str>) -> bool {
        let key = key.as_ref();
        self.updates.contains_key(key)
            || self.base.contains_key(key)
        // match self.updates.get(key.as_ref()) {
        //     Some(k) => k.is_some(),
        //     None => self.base.contains_key(key.as_ref()),
        // }
    }
    /// Look for a definition by name.
    pub fn get(&self, key: impl AsRef<str>) -> Option<&Val> {
        let key = key.as_ref();
        self.updates.get(key).or_else(|| self.base.get(key))
    }
    // /// An iterator over the contained definition names.
    // pub fn keys(&self) -> impl Iterator<Item = &str> {
    //     self.insertions.keys().chain(self.base.keys()).map(|k| k.borrow())
    // }
    // /// An iterator over the contained definition name/body pairs.
    // pub fn iter(&self) -> impl Iterator<Item = (&str, &Val)> {
    //     self.insertions.self.base.iter().map(|(k, v)| (k.borrow(), v))
    //     self.base.iter().map(|(k, v)| (k.borrow(), v))
    // }
    /// Whether there are no entries.
    pub fn is_empty(&self) -> bool {
        self.updates.is_empty() && self.base.is_empty()
    }
    // /// How many entries there are.
    // pub fn len(&self) -> usize { self.base.len() } // TODO

    // /// Retain definitions based on the given criterion.
    // pub fn filter<F: Fn(&str, &Val) -> bool>(&mut self, f: F) {
    //     Rc::make_mut(&mut self.base).retain(|k, v| f(k.as_ref(), v));
    // }

    /// Normalising a DefSet should make calls quicker.
    pub fn normalise(&mut self) {
        if self.updates.is_empty() { return; }
        let mut updates = Default::default();
        std::mem::swap(&mut self.updates, &mut updates);
        Rc::make_mut(&mut self.base).extend(updates.into_iter());
    }

    // merge this with that - that overwrites this
    fn merge(mut this: DefSet, that: DefSet) -> DefSet {
        if that.is_empty() { return this; }
        if this.is_empty() { return that; }
        if Rc::ptr_eq(&this.base, &that.base) {
            this.updates.extend(that.updates.into_iter());
            return this;
        }
        this.updates.extend(that.base.iter().map(|(k, v)| (k.to_string(), v.clone())));
        this.updates.extend(that.updates.into_iter());
        this.normalise();
        return this;
    }

    /// Take everything from `thee` and put it in `self`.
    /// This will overwrite existing values in `self`.
    pub fn append(&mut self, thee: &DefSet) {
        let mut tmp = DefSet::default();
        std::mem::swap(self, &mut tmp);
        *self = Self::merge(tmp, thee.clone());
    }
    /// Take everything from `thee` and put it in `self`,
    /// unless `self` already contains an entry with the same name.
    /// See also [append].
    pub fn prepend(&mut self, thee: &DefSet) {
        let mut tmp = DefSet::default();
        std::mem::swap(self, &mut tmp);
        *self = Self::merge(thee.clone(), tmp);
    }
}

