use super::*;
use std::ops::Index;

use std::cmp::Eq;
use std::hash::Hash;
use std::borrow::Borrow;

impl<'a, K: Eq + Hash + Borrow<Q>, V, Q: Eq + Hash + ?Sized> Index<&'a Q> for HashMap<K, V> {
    type Output = V;
    fn index(&self, index: &'a Q) -> &Self::Output {
        self.get(index).unwrap()
    }
}
