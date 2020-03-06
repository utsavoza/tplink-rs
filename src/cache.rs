use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

enum Status {
    NotFound,
    Found,
    Expired,
}

pub(crate) struct Cache<K, V> {
    store: HashMap<K, (Instant, V)>,
    ttl: Duration,
    hits: u32,
    misses: u32,
}

impl<K: Hash + Eq, V> Cache<K, V> {
    pub(crate) fn with_ttl(duration: Duration) -> Cache<K, V> {
        Cache {
            store: HashMap::new(),
            ttl: duration,
            hits: 0,
            misses: 0,
        }
    }

    pub(crate) fn with_ttl_and_capacity(duration: Duration, capacity: usize) -> Cache<K, V> {
        Cache {
            store: HashMap::with_capacity(capacity),
            ttl: duration,
            hits: 0,
            misses: 0,
        }
    }

    pub(crate) fn get<Q: ?Sized>(&mut self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let status = {
            let val = self.store.get(key);
            if let Some(&(instant, _)) = val {
                if instant.elapsed() < self.ttl {
                    Status::Found
                } else {
                    Status::Expired
                }
            } else {
                Status::NotFound
            }
        };
        match status {
            Status::NotFound => {
                self.misses += 1;
                None
            }
            Status::Found => {
                self.hits += 1;
                self.store.get(key).map(|(_, value)| value)
            }
            Status::Expired => {
                self.misses += 1;
                self.store.remove(key).unwrap();
                None
            }
        }
    }

    pub(crate) fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.store
            .insert(key, (Instant::now(), value))
            .map(|(_, value)| value)
    }

    pub(crate) fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.store.remove(key).map(|(_, value)| value)
    }

    pub(crate) fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.store
            .retain(|x: &K, y: &mut (Instant, V)| f(x, &mut y.1))
    }

    pub(crate) fn clear(&mut self) {
        self.store.clear();
    }

    pub(crate) fn hits(&self) -> Option<u32> {
        Some(self.hits)
    }

    pub(crate) fn misses(&self) -> Option<u32> {
        Some(self.misses)
    }

    pub(crate) fn ttl(&self) -> Option<Duration> {
        Some(self.ttl)
    }

    pub(crate) fn len(&self) -> usize {
        self.store.len()
    }
}
