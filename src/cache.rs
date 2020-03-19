use crate::error::Result;
use crate::proto::Request;

use serde_json::Value;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::hash::Hash;
use std::time::{Duration, Instant};

pub type ResponseCache = Option<Cache<Request, Value>>;

enum Status {
    NotFound,
    Found,
    Expired,
}

pub struct Cache<K, V> {
    store: HashMap<K, (Instant, V)>,
    ttl: Duration,
    hits: u32,
    misses: u32,
}

impl<K: Hash + Eq, V> Cache<K, V> {
    pub fn with_ttl(duration: Duration) -> Cache<K, V> {
        Cache {
            store: HashMap::new(),
            ttl: duration,
            hits: 0,
            misses: 0,
        }
    }

    pub fn with_ttl_and_capacity(duration: Duration, capacity: usize) -> Cache<K, V> {
        Cache {
            store: HashMap::with_capacity(capacity),
            ttl: duration,
            hits: 0,
            misses: 0,
        }
    }

    pub fn get<Q: ?Sized>(&mut self, key: &Q) -> Option<&V>
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

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.store
            .insert(key, (Instant::now(), value))
            .map(|(_, value)| value)
    }

    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.store.remove(key).map(|(_, value)| value)
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.store.retain(|k, v| f(k, &mut v.1))
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }

    pub fn hits(&self) -> Option<u32> {
        Some(self.hits)
    }

    pub fn misses(&self) -> Option<u32> {
        Some(self.misses)
    }

    pub fn ttl(&self) -> Option<Duration> {
        Some(self.ttl)
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    pub fn get_or_insert_with<F>(&mut self, key: K, f: F) -> Result<V>
    where
        F: Fn(&K) -> Result<V>,
        V: Clone,
    {
        match self.get(&key) {
            Some(value) => Ok(value.to_owned()),
            None => {
                let value = f(&key)?;
                self.insert(key, value.to_owned());
                Ok(value)
            }
        }
    }
}

impl<K, V> Debug for Cache<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cache")
            .field("ttl", &self.ttl)
            .field("hits", &self.hits)
            .field("misses", &self.misses)
            .finish()
    }
}
