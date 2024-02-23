const DEFAULT_SIZE: usize = 256;
pub struct MyHashmap<K, V> {
    // A guess at how many elements the hashmap will manage.
    size: usize,

    // The "table" which will hold all the data
    hash_elements: [Option<KeyValue<K, V>>; DEFAULT_SIZE],
}
#[derive(Clone, Debug)]
struct KeyValue<K, V> {
    key: K,
    value: V,
    next: Option<Box<KeyValue<K, V>>>,
}

impl<K: PartialEq + Clone, V: Clone> MyHashmap<K, V> {
    const INIT: Option<KeyValue<K, V>> = None;
    pub fn new() -> MyHashmap<K, V> {
        MyHashmap {
            size: DEFAULT_SIZE,
            hash_elements: [Self::INIT; DEFAULT_SIZE],
        }
    }
}

// TODO: Implement a real hash function
fn hash_key<T>(key: T) -> usize {
    return 1;
}
