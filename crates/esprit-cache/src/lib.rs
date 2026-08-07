use dashmap::DashMap;
use std::hash::Hash;

pub struct Cache<K: Eq + Hash, V> {
    inner: DashMap<K, V>,
}

impl<K: Eq + Hash, V> Default for Cache<K, V> {
    fn default() -> Self {
        Self { inner: DashMap::new() }
    }
}

impl<K: Eq + Hash, V> Cache<K, V> {
    pub fn insert(&self, k: K, v: V) {
        self.inner.insert(k, v);
    }

    pub fn get(&self, k: &K) -> Option<V>
    where
        V: Clone,
    {
        self.inner.get(k).map(|v| v.clone())
    }
}
