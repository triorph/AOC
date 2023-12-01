use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct HashVec<K, V>
where
    K: Eq + Hash,
{
    inner: HashMap<K, Vec<V>>,
}

pub trait FromVec<T> {
    fn from_vec(v: &[T]) -> Self;
}

impl<T> FromVec<T> for HashSet<T>
where
    T: Eq + Hash + Copy,
{
    fn from_vec(v: &[T]) -> Self {
        HashSet::from_iter(v.iter().copied())
    }
}

/// Provides a HashMap where the any key used is expected to have a vector
///
/// Will either return an empty vector, or auto-fill an empty vector when doing
/// changes.
impl<K, V> Default for HashVec<K, V>
where
    K: Eq + Hash + Copy,
    V: Clone,
{
    fn default() -> Self {
        HashVec::new()
    }
}

impl<K, V> HashVec<K, V>
where
    K: Eq + Hash + Copy,
    V: Clone,
{
    pub fn new() -> HashVec<K, V> {
        HashVec {
            inner: HashMap::new(),
        }
    }

    pub fn push(&mut self, key: K, val: V) {
        self.place_empty(key);
        self.inner.entry(key).and_modify(|v| v.push(val.clone()));
    }

    pub fn get(&self, key: &K) -> Vec<V> {
        self.inner.get(key).unwrap_or(&Vec::<V>::new()).clone()
    }

    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, K, Vec<V>> {
        self.inner.keys()
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut Vec<V>> {
        self.place_empty(*key);
        self.inner.get_mut(key)
    }

    pub fn extend(&mut self, key: K, values: &[V]) {
        self.place_empty(key);
        self.inner
            .entry(key)
            .and_modify(|v: &mut Vec<V>| v.extend(values.iter().cloned()));
    }

    fn place_empty(&mut self, key: K) {
        self.inner.entry(key).or_insert_with(|| Vec::new());
    }
}
