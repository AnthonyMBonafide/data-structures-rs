use std::{
    hash::{DefaultHasher, Hash, Hasher},
    u64,
};

const DEFAULT_SIZE: u64 = 256;
#[derive(Debug)]
pub struct MyHashmap<K, V> {
    // How many elements are in the map
    size: u64,

    // The "table" which will hold all the data
    hash_elements: Vec<Bucket<K, V>>,
    hasher: fn(key: K) -> usize,
}

#[derive(Debug, Clone)]
struct Bucket<K, V> {
    head: Option<KeyValue<K, V>>,
}

impl<'a, K, V> Bucket<K, V>
where
    K: PartialEq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn find(&self, key: K) -> Option<&V> {
        if self.head.is_none() {
            return None;
        }

        if self.head.as_ref().unwrap().key == key {
            return Some(&self.head.as_ref().unwrap().value);
        }
        // Loop through list
        let mut loop_variable = &self.head.as_ref().unwrap().next;
        while loop_variable.is_some() {
            if loop_variable.as_ref().unwrap().key == key {
                return Some(&loop_variable.as_ref().unwrap().value);
            }

            loop_variable = &(loop_variable.as_ref().unwrap()).next;
        }

        None
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.head.is_none() {
            self.head = Some(KeyValue::<K, V>::new(key, value, None));
            return;
        }

        if self.head.as_ref().unwrap().key == key {
            self.head.as_mut().unwrap().value = value;
            return;
        }
        // Loop through list
        let mut loop_variable = &mut self.head.as_mut().unwrap().next;
        while loop_variable.is_some() {
            if loop_variable.as_ref().unwrap().key == key {
                self.head.as_mut().unwrap().value = value;
                return;
            }

            if loop_variable.as_mut().unwrap().next.is_none() {
                loop_variable.as_mut().unwrap().next =
                    Some(Box::new(KeyValue::<K, V>::new(key, value, None)));

                return;
            }

            loop_variable = &mut loop_variable.as_mut().unwrap().next;
        }
    }
}
#[derive(Clone, Debug)]
struct KeyValue<K, V> {
    pub key: K,
    pub value: V,
    pub next: Option<Box<KeyValue<K, V>>>,
}

impl<K, V> KeyValue<K, V> {
    fn new(key: K, value: V, next: Option<Box<KeyValue<K, V>>>) -> Self {
        Self { key, value, next }
    }
}

impl<K, V> MyHashmap<K, V>
where
    K: PartialEq + Clone + Hash,
    V: Clone,
{
    pub fn new() -> MyHashmap<K, V> {
        MyHashmap {
            size: DEFAULT_SIZE,
            hash_elements: vec![Bucket::<K, V>::new(); DEFAULT_SIZE as usize],
            hasher: hash_key,
        }
    }

    pub fn with_capacity(size: u64) -> MyHashmap<K, V> {
        MyHashmap {
            size,
            hash_elements: vec![Bucket::<K, V>::new(); size as usize],
            hasher: hash_key,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let hash = (self.hasher)(key.clone());
        let bucket_index = hash % self.hash_elements.len();
        self.hash_elements[bucket_index].insert(key.clone(), value);
    }
}

// TODO: Implement a real hash function
fn hash_key<T: Hash>(key: T) -> usize {
    let mut h = DefaultHasher::new();
    key.hash(&mut h);
    h.finish() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_new() {
        let hm = MyHashmap::<String, i32>::new();
    }

    #[test]
    fn test_hashmap_insert() {
        let mut hm = MyHashmap::<String, i32>::with_capacity(10);
        hm.insert("hello".to_string(), 23);
        println!("{:?}", hm)
    }
    #[test]
    fn test_hashmap_insert_duplicate_key() {
        let mut hm = MyHashmap::<String, i32>::with_capacity(10);
        hm.insert("hello".to_string(), 23);
        hm.insert("hello".to_string(), 24);
        println!("{:?}", hm)
    }
}
