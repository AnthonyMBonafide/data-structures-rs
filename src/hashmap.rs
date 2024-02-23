use std::{
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
    u64,
};

const DEFAULT_SIZE: u64 = 256;
#[derive(Debug)]
pub struct MyHashmap<K, V> {
    // A guess at how many elements the hashmap will manage.
    size: u64,

    // The "table" which will hold all the data
    hash_elements: Vec<Bucket<K, V>>,
    hasher: fn(key: K) -> usize,
}
#[derive(Debug, Clone)]
struct Bucket<K, V> {
    head: Option<Rc<KeyValue<K, V>>>,
    tail: Option<Rc<KeyValue<K, V>>>,
}

impl<K, V> Bucket<K, V>
where
    K: PartialEq,
{
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn find(&self, key: K) -> Option<&V> {
        let mut c = &self.head;
        while let Some(kv) = c {
            if kv.key == key {
                return Some(&kv.value);
            }

            match &kv.next {
                Some(_) => c = &kv.next,
                None => c = &None,
            }
        }

        None
    }

    pub fn insert(&mut self, key: K, value: V) {
        let new_element = Rc::new(KeyValue::<K, V>::new(key, value, None));
        if self.tail.is_none() {
            self.head = Some(Rc::clone(&new_element));
            self.tail = Some(Rc::clone(&new_element));
        }

        self.tail = Some(Rc::clone(&new_element));
    }
}

#[derive(Clone, Debug)]
struct KeyValue<K, V> {
    pub key: K,
    pub value: V,
    pub next: Option<Rc<KeyValue<K, V>>>,
}

impl<K, V> KeyValue<K, V> {
    fn new(key: K, value: V, next: Option<Rc<KeyValue<K, V>>>) -> Self {
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
}
