use crate::cache::Cache;
use crate::crypto;
use crate::error::{self, Result};

use serde_json::{json, Value};
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::Duration;

struct CacheConfig {
    ttl: Duration,
    initial_capacity: Option<usize>,
}

impl CacheConfig {
    fn from(ttl: Duration, initial_capacity: Option<usize>) -> CacheConfig {
        CacheConfig {
            ttl,
            initial_capacity,
        }
    }
}

pub(crate) struct Builder {
    host: IpAddr,
    port: u16,
    buffer_size: usize,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
    cache_config: CacheConfig,
}

impl Builder {
    pub(crate) fn new<A>(host: A) -> Builder
    where
        A: Into<IpAddr>,
    {
        Builder {
            host: host.into(),
            port: 9999,
            buffer_size: 4096,
            read_timeout: None,
            write_timeout: None,
            cache_config: CacheConfig::from(Duration::from_secs(3), None),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn port(&mut self, port: u16) -> &mut Builder {
        self.port = port;
        self
    }

    #[allow(dead_code)]
    pub(crate) fn buffer_size(&mut self, buffer_size: usize) -> &mut Builder {
        self.buffer_size = buffer_size;
        self
    }

    pub(crate) fn read_timeout(&mut self, duration: Duration) -> &mut Builder {
        self.read_timeout = Some(duration);
        self
    }

    pub(crate) fn write_timeout(&mut self, duration: Duration) -> &mut Builder {
        self.write_timeout = Some(duration);
        self
    }

    pub(crate) fn cache_config(
        &mut self,
        ttl: Duration,
        initial_capacity: Option<usize>,
    ) -> &mut Builder {
        self.cache_config = CacheConfig::from(ttl, initial_capacity);
        self
    }

    pub(crate) fn build(&mut self) -> Proto {
        let CacheConfig {
            ttl,
            initial_capacity,
        } = self.cache_config;
        let cache = initial_capacity.map_or_else(
            || Cache::with_ttl(ttl),
            |capacity| Cache::with_ttl_and_capacity(ttl, capacity),
        );

        Proto {
            host: SocketAddr::new(self.host, self.port),
            buffer_size: self.buffer_size,
            read_timeout: self.read_timeout,
            write_timeout: self.write_timeout,
            cache,
        }
    }
}

pub(crate) struct Proto {
    host: SocketAddr,
    buffer_size: usize,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
    cache: Cache<String, Vec<u8>>,
}

impl Proto {
    pub(crate) fn send(&mut self, target: &str, cmd: &str, arg: Option<&Value>) -> Result<Vec<u8>> {
        let req = format!("{}:{}", target, cmd);
        match self.cache.get(&req) {
            Some(res) => Ok(res.clone()),
            None => {
                let res = self.send_bytes(
                    &serde_json::to_vec(&json!({target:{cmd:arg}}))
                        .map(|bytes| crypto::encrypt(&bytes))
                        .map_err(error::json)?,
                )?;
                self.cache.insert(req, res.clone());
                Ok(res)
            }
        }
    }

    fn send_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        socket.set_read_timeout(self.read_timeout)?;
        socket.set_write_timeout(self.write_timeout)?;
        socket.send_to(bytes, self.host)?;

        let mut buf = vec![0; self.buffer_size];
        match socket.recv(&mut buf) {
            Ok(recv) => Ok(crypto::decrypt(&buf[..recv])),
            Err(e) => Err(e.into()),
        }
    }
}
