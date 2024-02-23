use std::u64;

const DEFAULT_SIZE: u64 = 256;
pub struct MyHashmap<K, V> {
    // A guess at how many elements the hashmap will manage.
    size: u64,

    // The "table" which will hold all the data
    hash_elements: Vec<Option<KeyValue<K, V>>>,
    hasher: fn(key: K) -> usize,
}
#[derive(Clone, Debug)]
struct KeyValue<K, V> {
    key: K,
    value: V,
    next: Option<Box<KeyValue<K, V>>>,
}

impl<K, V> MyHashmap<K, V>
where
    K: PartialEq + Clone,
    V: Clone,
{
    const INIT: Option<KeyValue<K, V>> = None;
    pub fn new() -> MyHashmap<K, V> {
        MyHashmap {
            size: DEFAULT_SIZE,
            hash_elements: vec![Self::INIT; DEFAULT_SIZE as usize],
            hasher: hash_key,
        }
    }

    pub fn with_capacity(size: u64) -> MyHashmap<K, V> {
        MyHashmap {
            size,
            hash_elements: vec![Self::INIT; size as usize],
            hasher: hash_key,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let hash = (self.hasher)(key);
        let bucket_index = hash % self.hash_elements.len();

        match self.hash_elements[bucket_index] {
            // TODO: handle the data. We should create a new field in the bucket which holds a
            // pointer to the last element in the bucket for quick insertions
            Some(_) => println!("Some"),
            None => println!("Nothing"),
        }
    }
}

impl<K, V> Default for MyHashmap<K, V>
where
    K: PartialEq + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Implement a real hash function
fn hash_key<T>(key: T) -> usize {
    return 1;
}
