use std::cmp::Eq;
use std::hash::Hash;

use super::*;

pub enum Entry<'a, K: Eq + Hash, V> {
    Vacant(VacantEntry<'a, K, V>),
    Occupied(OccupiedEntry<'a, K, V>),
}

impl<'a, K: Eq + Hash, V> Entry<'a, K, V> {
    pub fn new(map: &'a mut HashMap<K, V>, key: K) -> Entry<'a, K, V> {
        if map.contains_key(&key) {
            let v = map.get_mut(&key).unwrap();
            Self::Occupied(OccupiedEntry { key, value: v })
        } else {
            Self::Vacant(VacantEntry { key, map })
        }
    }
    pub fn or_insert(self, val: V) -> &'a mut V {
        match self {
            Entry::Vacant(ventry) => ventry.map.insert_mut(ventry.key, val),
            Entry::Occupied(oentry) => oentry.value,
        }
    }

    pub fn or_insert_with<F: FnOnce() -> V>(self, f: F) -> &'a mut V {
        match self {
            Entry::Vacant(ventry) => ventry.map.insert_mut(ventry.key, f()),
            Entry::Occupied(oentry) => oentry.value,
        }
    }
}

pub struct VacantEntry<'a, K: Eq + Hash, V> {
    map: &'a mut HashMap<K, V>,
    key: K,
}

impl<'a, K: Eq + Hash, V> VacantEntry<'a, K, V> {
    pub fn insert(self, value: V) -> &'a mut V {
        self.map.insert_mut(self.key, value)
    }
}

pub struct OccupiedEntry<'a, K, V> {
    key: K,
    value: &'a mut V,
}

#[cfg(test)]
mod tests {
    use crate::Bucket;
    use crate::Entry;
    use crate::HashMap;

    #[test]
    fn entry_api() {
        let mut m: HashMap<u64, String> = HashMap::new();
        let e0 = m.entry(3);
        match e0 {
            Entry::Occupied(_) => unreachable!(),
            Entry::Vacant(_) => (),
        };
        e0.or_insert("hi".to_string());
        assert_eq!(m[&3], "hi".to_string());
        assert!(m.contains_key(&3));
        let e1 = m.entry(3);
        match e1 {
            Entry::Occupied(_) => (),
            Entry::Vacant(_) => unreachable!(),
        };
        // should not replace since entry occupied
        e1.or_insert("hi0".to_string());
        assert_eq!(m[&3], "hi".to_string());
        m.entry(2).or_insert_with(|| "hi".to_string());
        assert_eq!(m[&2], "hi".to_string());
    }
}
