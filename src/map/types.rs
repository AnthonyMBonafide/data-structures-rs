pub trait Map<K, V> {
    fn insert(&mut self, key: K, value: V);

    fn get(&self, key: &K) -> Option<&V>;
    fn clear(&mut self);

    fn remove(&mut self, key: &K);
}
