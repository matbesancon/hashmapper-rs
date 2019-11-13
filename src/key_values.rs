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

pub struct ValuesMut<'a, K: Eq + Hash, V> {
    bucket_idx: usize,
    bucket_at: usize,
    hmap: &'a mut HashMap<K, V>,
}

impl<'a, K: Eq + Hash, V> ValuesMut<'a, K, V> {
    pub fn new(map: &'a mut HashMap<K, V>) -> Self {
        ValuesMut {
            hmap: map,
            bucket_idx: 0,
            bucket_at: 0,
        }
    }
}

impl<'a, K: Eq + Hash, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;
    fn next(&mut self) -> Option<Self::Item> {
        if self.hmap.num_items == 0 {
            return None;
        }
        
        // match self.hmap.buckets.get_mut(self.bucket_idx) {
        //     None => None, // no more bucket
        //     Some(bkt) => {
        //         let new_pair: Option<&mut (K,V)> = bkt.at_mut(self.bucket_at);
        //         match new_pair {
        //             None => {
        //                 // end of bucket, switch to next
        //                 self.bucket_at = 0;
        //                 self.bucket_idx += 1;
        //                 self.next()
        //             }
        //             Some(p) => {
        //                 // still some in current bucket
        //                 self.bucket_at += 1;
        //                 let (_, ref mut v) = p;
        //                 Some(v)
        //             }
        //         }
        //     }
        // }
        None
    }
}
