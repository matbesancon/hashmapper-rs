use super::*;

use std::cmp::Eq;
use std::hash::Hash;

/// Iterator on the keys of a hash map.
pub struct Keys<'a, K: Eq + Hash, V> {
    map_iter: HashMapIterator<'a, K, V>,
}

impl<'a, K: Eq + Hash, V> Keys<'a, K, V> {
    pub fn new(map: &'a HashMap<K, V>) -> Self {
        Keys {
            map_iter: map.into_iter(),
        }
    }
}

impl<'a, K: Eq + Hash, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;
    fn next(&mut self) -> Option<Self::Item> {
        let (k, _) = self.map_iter.next()?;
        Some(k)
    }
}

/// Iterator on the values of a hash map.
pub struct Values<'a, K: Eq + Hash, V> {
    map_iter: HashMapIterator<'a, K, V>,
}

impl<'a, K: Eq + Hash, V> Values<'a, K, V> {
    pub fn new(map: &'a HashMap<K, V>) -> Self {
        Values {
            map_iter: map.into_iter(),
        }
    }
}

impl<'a, K: Eq + Hash, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        let (_, v) = self.map_iter.next()?;
        Some(v)
    }
}
