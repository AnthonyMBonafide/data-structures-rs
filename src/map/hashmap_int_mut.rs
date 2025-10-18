use super::types::Map;

pub struct HashMapIntMut<K, V> {
    key: K,
    valuse: V,
}

impl<K, V> Map<K, V> for HashMapIntMut<K, V> {
    fn insert(&mut self, key: K, value: V) {
        todo!()
    }

    fn get(&self, key: &K) -> Option<&V> {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn remove(&mut self, key: &K) {
        todo!()
    }
}
