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
        MyHashmap::with_capacity(DEFAULT_SIZE)
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

    pub fn remove(&mut self, key: K) {
        let hash = (self.hasher)(key.clone());
        let bucket_index = hash % self.hash_elements.len();
        self.hash_elements[bucket_index].remove(key.clone());
    }
}

#[derive(Debug, Clone)]
struct Bucket<K, V> {
    head: Option<Box<KeyValue<K, V>>>,
}

impl<K, V> Bucket<K, V>
where
    K: PartialEq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn find(&self, key: K) -> Option<&V> {
        self.head.as_ref()?;

        let head_kv = self.head.as_ref().unwrap();
        if head_kv.key == key {
            return Some(&head_kv.value);
        }
        // Loop through list
        let mut next_kv = &head_kv.next;
        while next_kv.is_some() {
            let current_kv = next_kv.as_ref().unwrap();
            if next_kv.as_ref().unwrap().key == key {
                return Some(&current_kv.value);
            }

            next_kv = &current_kv.next;
        }

        None
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.head.is_none() {
            self.head = Some(Box::new(KeyValue::<K, V>::new(key, value, None)));
            return;
        }

        let h = self.head.as_mut().unwrap();
        if h.key == key {
            h.value = value;
            return;
        }

        if h.next.is_none() {
            h.next = Some(Box::new(KeyValue::<K, V>::new(key, value, None)));
            return;
        }
        // Loop through list
        let mut next_kv = &mut h.next;
        while next_kv.is_some() {
            let current_kv = next_kv.as_mut().unwrap();
            if current_kv.key == key {
                h.value = value;
                return;
            }

            if current_kv.next.is_none() {
                current_kv.next = Some(Box::new(KeyValue::<K, V>::new(key, value, None)));

                return;
            }

            next_kv = &mut current_kv.next;
        }
    }

    pub fn remove(&mut self, key: K) {
        if self.head.is_none() {
            return;
        }

        let h = self.head.as_ref().unwrap();
        if h.key == key {
            self.head = h.next.to_owned();
            return;
        }

        let mut previous = self.head.to_owned();
        let mut current = previous.as_ref().unwrap().as_ref().next.to_owned();

        while current.is_some() {
            if current.as_ref().unwrap().key == key {
                previous
                    .as_mut()
                    .unwrap()
                    .next(current.as_mut().unwrap().next.to_owned());
            }

            previous = current;
            current = previous.as_mut().unwrap().next.to_owned();
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

    fn next(&mut self, next: Option<Box<KeyValue<K, V>>>) {
        self.next = next;
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

    #[test]
    fn test_hashmap_clear() {
        let mut hm = MyHashmap::new();
        hm.insert("test".to_string(), 1);
        hm.insert("test1".to_string(), 1);
        hm.insert("test2".to_string(), 1);
        hm.insert("test3".to_string(), 1);
        hm.clear();
        assert!(hm.get("test".to_string()).is_none());
        assert!(hm.get("test1".to_string()).is_none());
        assert!(hm.get("test2".to_string()).is_none());
        assert!(hm.get("test3".to_string()).is_none());
    }

    #[test]
    fn hashmap_remove() {
        let mut hm = MyHashmap::new();
        hm.insert("test".to_string(), 1);
        assert!(hm.get("test".to_string()).is_some());
        hm.remove("test".to_string());
        assert!(hm.get("test".to_string()).is_none());
    }
    #[test]
    fn test_hashmap_remove_many() {
        let mut hm = MyHashmap::with_capacity(10);
        hm.insert("test".to_string(), 1);
        hm.insert("test1".to_string(), 1);
        hm.insert("test2".to_string(), 1);
        hm.insert("test3".to_string(), 1);
        hm.insert("test4".to_string(), 1);
        hm.insert("test5".to_string(), 1);
        hm.insert("test6".to_string(), 1);
        hm.insert("test7".to_string(), 1);
        hm.insert("test8".to_string(), 1);
        hm.insert("test9".to_string(), 1);
        hm.insert("test10".to_string(), 1);
        hm.insert("test11".to_string(), 1);
        assert!(hm.get("test".to_string()).is_some());
        assert!(hm.get("test1".to_string()).is_some());
        assert!(hm.get("test2".to_string()).is_some());
        assert!(hm.get("test3".to_string()).is_some());
        assert!(hm.get("test4".to_string()).is_some());
        assert!(hm.get("test5".to_string()).is_some());
        assert!(hm.get("test6".to_string()).is_some());
        assert!(hm.get("test7".to_string()).is_some());
        assert!(hm.get("test8".to_string()).is_some());
        assert!(hm.get("test9".to_string()).is_some());
        assert!(hm.get("test10".to_string()).is_some());
        assert!(hm.get("test11".to_string()).is_some());

        hm.remove("test".to_string());
        assert!(hm.get("test".to_string()).is_none());
        assert!(hm.get("test1".to_string()).is_some());
        assert!(hm.get("test2".to_string()).is_some());
        assert!(hm.get("test3".to_string()).is_some());
        assert!(hm.get("test4".to_string()).is_some());
        assert!(hm.get("test5".to_string()).is_some());
        assert!(hm.get("test6".to_string()).is_some());
        assert!(hm.get("test7".to_string()).is_some());
        assert!(hm.get("test8".to_string()).is_some());
        assert!(hm.get("test9".to_string()).is_some());
        assert!(hm.get("test10".to_string()).is_some());
        assert!(hm.get("test11".to_string()).is_some());

        hm.remove("test1".to_string());
        assert!(hm.get("test1".to_string()).is_none());
        assert!(hm.get("test2".to_string()).is_some());
        assert!(hm.get("test3".to_string()).is_some());
        assert!(hm.get("test4".to_string()).is_some());
        assert!(hm.get("test5".to_string()).is_some());
        assert!(hm.get("test6".to_string()).is_some());
        assert!(hm.get("test7".to_string()).is_some());
        assert!(hm.get("test8".to_string()).is_some());
        assert!(hm.get("test9".to_string()).is_some());
        assert!(hm.get("test10".to_string()).is_some());
        assert!(hm.get("test11".to_string()).is_some());

        hm.remove("test2".to_string());
        assert!(hm.get("test2".to_string()).is_none());
        assert!(hm.get("test3".to_string()).is_some());
        assert!(hm.get("test4".to_string()).is_some());
        assert!(hm.get("test5".to_string()).is_some());
        assert!(hm.get("test6".to_string()).is_some());
        assert!(hm.get("test7".to_string()).is_some());
        assert!(hm.get("test8".to_string()).is_some());
        assert!(hm.get("test9".to_string()).is_some());
        assert!(hm.get("test10".to_string()).is_some());
        assert!(hm.get("test11".to_string()).is_some());

        hm.remove("test3".to_string());
        assert!(hm.get("test3".to_string()).is_none());
        assert!(hm.get("test4".to_string()).is_some());
        assert!(hm.get("test5".to_string()).is_some());
        assert!(hm.get("test6".to_string()).is_some());
        assert!(hm.get("test7".to_string()).is_some());
        assert!(hm.get("test8".to_string()).is_some());
        assert!(hm.get("test9".to_string()).is_some());
        assert!(hm.get("test10".to_string()).is_some());
        assert!(hm.get("test11".to_string()).is_some());

        hm.remove("test4".to_string());
        assert!(hm.get("test4".to_string()).is_none());
        assert!(hm.get("test5".to_string()).is_some());
        assert!(hm.get("test6".to_string()).is_some());
        assert!(hm.get("test7".to_string()).is_some());
        assert!(hm.get("test8".to_string()).is_some());
        assert!(hm.get("test9".to_string()).is_some());
        assert!(hm.get("test10".to_string()).is_some());
        assert!(hm.get("test11".to_string()).is_some());

        hm.remove("test5".to_string());
        assert!(hm.get("test5".to_string()).is_none());
        assert!(hm.get("test6".to_string()).is_some());
        assert!(hm.get("test7".to_string()).is_some());
        assert!(hm.get("test8".to_string()).is_some());
        assert!(hm.get("test9".to_string()).is_some());
        assert!(hm.get("test10".to_string()).is_some());
        assert!(hm.get("test11".to_string()).is_some());

        hm.remove("test6".to_string());
        assert!(hm.get("test6".to_string()).is_none());
        assert!(hm.get("test7".to_string()).is_some());
        assert!(hm.get("test8".to_string()).is_some());
        assert!(hm.get("test9".to_string()).is_some());
        assert!(hm.get("test10".to_string()).is_some());
        assert!(hm.get("test11".to_string()).is_some());

        hm.remove("test7".to_string());
        assert!(hm.get("test7".to_string()).is_none());
        assert!(hm.get("test8".to_string()).is_some());
        assert!(hm.get("test9".to_string()).is_some());
        assert!(hm.get("test10".to_string()).is_some());
        assert!(hm.get("test11".to_string()).is_some());

        hm.remove("test8".to_string());
        assert!(hm.get("test8".to_string()).is_none());
        assert!(hm.get("test9".to_string()).is_some());
        assert!(hm.get("test10".to_string()).is_some());
        assert!(hm.get("test11".to_string()).is_some());

        hm.remove("test9".to_string());
        assert!(hm.get("test9".to_string()).is_none());
        assert!(hm.get("test10".to_string()).is_some());
        assert!(hm.get("test11".to_string()).is_some());

        hm.remove("test10".to_string());
        assert!(hm.get("test10".to_string()).is_none());
        assert!(hm.get("test11".to_string()).is_some());

        hm.remove("test11".to_string());
        assert!(hm.get("test11".to_string()).is_none());
    }
}
