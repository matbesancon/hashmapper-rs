use std::borrow::Borrow;
use std::cmp::Eq;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::mem;

mod bucket;
mod entry;
mod indexing;
mod key_values;

use bucket::*;
use entry::*;
use key_values::*;

/// Associative data structure
pub struct HashMap<K: Eq + Hash, V> {
    buckets: Vec<Bucket<K, V>>,
    num_items: usize,
}

impl<K: Eq + Hash, V> Default for HashMap<K, V> {
    fn default() -> Self {
        HashMap {
            buckets: Vec::new(),
            num_items: 0,
        }
    }
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let idx = (hasher.finish() % self.buckets.len() as u64) as usize;
        self.buckets[idx].get(key.borrow())
    }

    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let idx = (hasher.finish() % self.buckets.len() as u64) as usize;
        self.buckets[idx].get_mut(key.borrow())
    }

    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
    {
        if self.num_items == 0 {
            return false;
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let idx = (hasher.finish() % self.buckets.len() as u64) as usize;
        self.buckets[idx].contains_key(key)
    }

    fn get_bucket_mut(&mut self, key: &K) -> &mut Bucket<K, V> {
        if self.buckets.is_empty() {
            self.resize();
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let idx = (hasher.finish() % self.buckets.len() as u64) as usize;
        &mut self.buckets[idx]
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let l = self.buckets.len();
        if l == 0 || self.num_items > 3 * l {
            self.resize();
        }
        self.num_items += 1;
        self.get_bucket_mut(&key).insert(key, value)
    }

    pub fn insert_mut(&mut self, key: K, value: V) -> &mut V {
        let l = self.buckets.len();
        if l == 0 || self.num_items > 3 * l {
            self.resize();
        }
        self.num_items += 1;
        self.get_bucket_mut(&key).insert_mut(key, value)
    }

    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let idx = (hasher.finish() % self.buckets.len() as u64) as usize;
        let bucket = &mut self.buckets[idx];
        let res = bucket.remove(key);
        if res.is_some() {
            self.num_items -= 1;
        }
        res
    }

    pub fn len(&self) -> usize {
        self.num_items
    }

    pub fn is_empty(&self) -> bool {
        self.num_items == 0
    }

    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => 1024,
            n => 2 * n,
        };
        let mut hasher = DefaultHasher::new();
        let mut new_buckets: Vec<Bucket<K, V>> = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Bucket::new()));
        for (key, value) in self.buckets.iter_mut().flat_map(|bkt| bkt.items.drain(..)) {
            key.hash(&mut hasher);
            let idx = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[idx].insert(key, value);
        }
        mem::replace(&mut self.buckets, new_buckets);
    }

    pub fn keys(&self) -> Keys<K, V> {
        Keys::new(self)
    }

    pub fn values(&self) -> Values<K, V> {
        Values::new(self)
    }

    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        ValuesMut::new(self)
    }

    pub fn entry<'a>(&'a mut self, key: K) -> Entry<'_, K, V> {
        Entry::new(self, key)
    }
}

pub struct HashMapIterator<'a, K: Eq + Hash, V> {
    hmap: &'a HashMap<K, V>,
    bucket_idx: usize,
    bucket_at: usize,
}

impl<'a, K: Eq + Hash, V> HashMapIterator<'a, K, V> {
    pub fn new(hm: &'a HashMap<K, V>) -> Self {
        HashMapIterator {
            hmap: hm,
            bucket_idx: 0,
            bucket_at: 0,
        }
    }
}

impl<'a, K: Eq + Hash, V> Iterator for HashMapIterator<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        if self.hmap.num_items == 0 {
            return None;
        }
        loop {
            match self.hmap.buckets.get(self.bucket_idx) {
                None => break None, // no more bucket
                Some(bkt) => {
                    let new_pair = bkt.at(self.bucket_at);
                    match new_pair {
                        None => {
                            // end of bucket, switch to next
                            self.bucket_at = 0;
                            self.bucket_idx += 1;
                        }
                        Some(p) => {
                            // still some in current bucket
                            self.bucket_at += 1;
                            break Some(p).map(|(k, v)| (k, v));
                        }
                    }
                }
            }
        }
    }
}

impl<'a, K: Eq + Hash, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = HashMapIterator<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        HashMapIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::Bucket;
    use crate::HashMap;

    #[test]
    fn create_insert() {
        let mut m: HashMap<u64, String> = HashMap::new();
        assert_eq!(m.num_items, 0);
        assert!(m.insert(3, "hi".to_string()).is_none());
        assert_eq!(m.num_items, 1);
    }

    #[test]
    fn insert_get_remove() {
        let mut m: HashMap<u64, String> = HashMap::new();
        assert!(m.insert(3, "hi".to_string()).is_none());
        assert_eq!(m.get(&3), Some(&"hi".to_string()));
        assert_eq!(m.remove(&3), Some("hi".to_string()));
        assert_eq!(m.len(), 0)
    }

    #[test]
    fn iter_on_bucket() {
        let mut bkt: Bucket<u64, String> = Bucket::new();
        assert!(bkt.insert(3, "hi".to_string()).is_none());
        assert!(bkt.insert(2, "hi".to_string()).is_none());
        assert!(bkt.insert(1, "hi".to_string()).is_none());
        let mut nitems = 0;
        for (k, v) in bkt.into_iter() {
            nitems += 1;
            assert!(k <= &3);
            assert_eq!(v, &"hi".to_string());
        }
        assert_eq!(nitems, 3);
    }

    #[test]
    fn iter_on_hashmap() {
        let mut m: HashMap<u64, String> = HashMap::new();
        assert_eq!(m.insert(3, "hi".to_string()), None);
        assert_eq!(m.insert(4, "hi".to_string()), None);
        assert_eq!(m.insert(5, "hi".to_string()), None);
        assert_eq!(m.insert(0, "hi".to_string()), None);
        let mut items = 0;
        for (k, v) in m.into_iter() {
            items += 1;
            assert!(k <= &5);
            assert_eq!(v, &"hi".to_string());
        }
        assert_eq!(items, 4);
    }
}
