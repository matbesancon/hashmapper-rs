use std::cmp::Eq;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

/// Storage in a hash map
/// All elements in a bucket are at the same hash.
#[derive(Clone, Debug)]
struct Bucket<K: Eq + Hash, V> {
    items: Vec<(K, V)>,
}

impl<K: Eq + Hash, V> Bucket<K, V> {
    fn new() -> Bucket<K, V> {
        Bucket { items: Vec::new() }
    }

    fn get(&self, key: K) -> Option<&V> {
        for (k, v) in self.items.iter() {
            if k == &key {
                return Some(v);
            }
        }
        None
    }
    fn contains_key(&self, key: K) -> bool {
        self.items.iter().any(|&(ref k, _)| k == &key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        for &mut (ref ekey, ref mut evalue) in self.items.iter_mut() {
            if ekey == &key {
                return Some(mem::replace(evalue, value));
            }
        }
        self.items.push((key, value));
        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let i = self.items.iter().position(|&(ref k, _)| k == key)?;
        Some(self.items.swap_remove(i).1)
    }

    fn at(&self, idx: usize) -> Option<&(K,V)> {
        self.items.get(idx)
    }
}

impl<'a, K: Eq + Hash, V> IntoIterator for &'a Bucket<K, V> {
    type Item = &'a (K, V);
    type IntoIter = std::slice::Iter<'a, (K, V)>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

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
        HashMap {
            buckets: Vec::new(),
            num_items: 0,
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let idx = (hasher.finish() % self.buckets.len() as u64) as usize;
        self.buckets[idx].get(key)
    }

    pub fn contains_key(&self, key: K) -> bool {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let idx = (hasher.finish() % self.buckets.len() as u64) as usize;
        self.buckets[idx].contains_key(key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let l = self.buckets.len();
        if l == 0 || self.num_items > 3 * l {
            self.resize();
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let idx = (hasher.finish() % self.buckets.len() as u64) as usize;
        let bucket = &mut self.buckets[idx];
        self.num_items += 1;
        bucket.insert(key, value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
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
        // println!("current state: num items {:?}, is_none {:?}", self.hmap.num_items, self.bucket_iter.is_none());
        // println!("bucket_idx: {:?}", self.bucket_idx);
        // empty hashmap
        if self.hmap.num_items == 0 {
            return None;
        }
        match self.hmap.buckets.get(self.bucket_idx) {
            None => None, // no more bucket
            Some(bkt) => {
                let new_pair = bkt.at(self.bucket_at);
                match new_pair {
                    None => { // end of bucket, switch to next
                        self.bucket_at = 0;
                        self.bucket_idx += 1;
                        self.next()
                    },
                    Some(p) => { // still some in current bucket
                        self.bucket_at += 1;
                        Some(p)
                            .map(|(k, v)| (k, v))
                    },
                }
            },
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
        assert_eq!(m.get(3), Some(&"hi".to_string()));
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
