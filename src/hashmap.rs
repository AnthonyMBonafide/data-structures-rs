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
        self.hash_elements[bucket_index].insert(key.clone(), value.clone());
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let hash = (self.hasher)(key.clone());
        let bucket_index = hash % self.hash_elements.len();
        self.hash_elements[bucket_index].find(key.clone())
    }
    pub fn clear(&mut self) {
        self.hash_elements.clear();
        self.hash_elements = vec![Bucket::<K, V>::new(); self.size as usize];
    }
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
        self.head.as_ref()?;

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

        if self.head.as_ref().unwrap().next.is_none() {
            self.head.as_mut().unwrap().next =
                Some(Box::new(KeyValue::<K, V>::new(key, value, None)));
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
        let _hm = MyHashmap::<String, i32>::new();
    }

    #[test]
    fn test_hashmap_insert() {
        let mut hm = MyHashmap::<String, i32>::with_capacity(10);
        hm.insert("hello".to_string(), 23);
        let result = hm.get("hello".to_string());
        assert!(result.is_some());
        assert_eq!(*result.unwrap(), 23);
    }
    #[test]
    fn test_hashmap_insert_duplicate_key() {
        let mut hm = MyHashmap::<String, i32>::with_capacity(10);
        hm.insert("hello".to_string(), 23);
        hm.insert("hello".to_string(), 24);
        let result = hm.get("hello".to_string());
        assert!(result.is_some());
        assert_eq!(*result.unwrap(), 24);
    }
    #[test]
    fn test_hashmap_collisions() {
        let mut hm = MyHashmap::<String, i32>::with_capacity(10);
        hm.insert("hello1".to_string(), 1);
        hm.insert("hello2".to_string(), 2);
        hm.insert("hello3".to_string(), 3);
        hm.insert("hello4".to_string(), 4);
        hm.insert("hello5".to_string(), 5);
        hm.insert("hello6".to_string(), 6);
        hm.insert("hello7".to_string(), 7);
        hm.insert("hello8".to_string(), 8);
        hm.insert("hello9".to_string(), 9);
        hm.insert("hello10".to_string(), 10);
        hm.insert("hello11".to_string(), 11);
        hm.insert("hello12".to_string(), 12);
        assert_eq!(*hm.get("hello1".to_string()).unwrap(), 1);
        assert_eq!(*hm.get("hello2".to_string()).unwrap(), 2);
        assert_eq!(*hm.get("hello3".to_string()).unwrap(), 3);
        assert_eq!(*hm.get("hello4".to_string()).unwrap(), 4);
        assert_eq!(*hm.get("hello5".to_string()).unwrap(), 5);
        assert_eq!(*hm.get("hello6".to_string()).unwrap(), 6);
        assert_eq!(*hm.get("hello7".to_string()).unwrap(), 7);
        assert_eq!(*hm.get("hello8".to_string()).unwrap(), 8);
        assert_eq!(*hm.get("hello9".to_string()).unwrap(), 9);
        assert_eq!(*hm.get("hello10".to_string()).unwrap(), 10);
        assert_eq!(*hm.get("hello11".to_string()).unwrap(), 11);
        assert_eq!(*hm.get("hello12".to_string()).unwrap(), 12);
    }
}
