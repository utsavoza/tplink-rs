use crate::cache::Cache;
use crate::crypto;
use crate::error::{self, Result};

use serde_json::{json, Value};
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::Duration;

pub(crate) struct Builder {
    host: IpAddr,
    port: u16,
    buffer_size: usize,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
    cache_config: Option<CacheConfig>,
}

struct CacheConfig(Duration, Option<usize>);

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
            cache_config: None,
        }
    }

    pub(crate) fn port(&mut self, port: u16) -> &mut Builder {
        self.port = port;
        self
    }

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

    pub(crate) fn enable_cache(
        &mut self,
        ttl: Duration,
        initial_capacity: Option<usize>,
    ) -> &mut Builder {
        self.cache_config = Some(CacheConfig(ttl, initial_capacity));
        self
    }

    pub(crate) fn build(&mut self) -> Proto {
        let cache = self.cache_config.as_ref().map_or_else(
            || None,
            |CacheConfig(ttl, initial_capacity)| {
                initial_capacity.map_or_else(
                    || Some(Cache::with_ttl(*ttl)),
                    |capacity| Some(Cache::with_ttl_and_capacity(*ttl, capacity)),
                )
            },
        );

        Proto {
            host: SocketAddr::new(self.host, self.port),
            buffer_size: self.buffer_size,
            read_timeout: self.read_timeout,
            write_timeout: self.write_timeout,
            response_cache: cache,
        }
    }
}

pub(crate) struct Proto {
    host: SocketAddr,
    buffer_size: usize,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
    response_cache: Option<Cache<String, Vec<u8>>>,
}

impl Proto {
    pub(crate) fn send(&self, target: &str, cmd: &str, arg: Option<&Value>) -> Result<Vec<u8>> {
        let bytes = serde_json::to_vec(&json!({target:{cmd:arg}}))
            .map(|bytes| crypto::encrypt(&bytes))
            .map_err(error::json)?;
        self.send_bytes(&bytes)
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
