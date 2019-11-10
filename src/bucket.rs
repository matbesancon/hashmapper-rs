use std::borrow::Borrow;
use std::cmp::Eq;
use std::hash::Hash;

use std::mem;

/// Storage in a hash map
/// All elements in a bucket are at the same hash.
#[derive(Clone, Debug)]
pub struct Bucket<K: Eq + Hash, V> {
    pub items: Vec<(K, V)>,
}

impl<K: Eq + Hash, V> Default for Bucket<K, V> {
    fn default() -> Self {
        Bucket { items: Vec::new() }
    }
}

impl<K: Eq + Hash, V> Bucket<K, V> {
    pub fn new() -> Bucket<K, V> {
        Self::default()
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
    {
        for (k, v) in self.items.iter() {
            if k.borrow() == key {
                return Some(v);
            }
        }
        None
    }
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
    {
        self.items.iter().any(|&(ref k, _)| k.borrow() == key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        for &mut (ref ekey, ref mut evalue) in self.items.iter_mut() {
            if ekey == &key {
                return Some(mem::replace(evalue, value));
            }
        }
        self.items.push((key, value));
        None
    }

    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
    {
        let i = self
            .items
            .iter()
            .position(|&(ref k, _)| k.borrow() == key)?;
        Some(self.items.swap_remove(i).1)
    }

    pub fn at(&self, idx: usize) -> Option<&(K, V)> {
        self.items.get(idx)
    }

    pub fn at_mut(&mut self, idx: usize) -> Option<&mut (K, V)> {
        self.items.get_mut(idx)
    }
}

impl<'a, K: Eq + Hash, V> IntoIterator for &'a Bucket<K, V> {
    type Item = &'a (K, V);
    type IntoIter = std::slice::Iter<'a, (K, V)>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
