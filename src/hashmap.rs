const DEFAULT_SIZE: usize = 50;
#[derive(Debug)]
pub struct HashMap<K, V>
where
    K: Clone + PartialEq,
{
    size: usize,

    elements: Vec<Vec<(K, V)>>,
    hasher: fn(&K) -> usize,
}

impl<K, V> HashMap<K, V>
where
    K: Clone + PartialEq,
{
    pub fn new() -> Self {
        HashMap {
            size: DEFAULT_SIZE,
            elements: Vec::new(),
            hasher: default_hasher,
        }
    }

    //TODO: Ensure we check if the key already exists and overwrite it
    pub fn insert(&mut self, key: &K, value: V) {
        let hashkey = (self.hasher)(key);
        let index = hashkey % self.size;
        self.elements[index].push((key.clone(), value));
    }

    fn get(&self, key: &K) -> Option<&V> {
        let hashkey = (self.hasher)(key);
        let index = hashkey % self.size;
        let collision_bucket: &Vec<(K, V)> = self.elements[index].as_ref();

        match collision_bucket.iter().find(|(k, _)| k == key) {
            Some((_, v)) => Some(v),
            None => None,
        }
    }
}

//TODO: figure out how to hash properly
fn default_hasher<K>(input: &K) -> usize {
    return 1;
}
