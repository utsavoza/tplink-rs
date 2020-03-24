use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

#[derive(Debug)]
pub struct Config {
    pub(crate) addr: SocketAddr,
    pub(crate) read_timeout: Duration,
    pub(crate) write_timeout: Duration,
    pub(crate) cache_config: CacheConfig,
    pub(crate) buffer_size: usize,
}

impl Config {
    pub fn for_host<A>(addr: A) -> ConfigBuilder
    where
        A: Into<IpAddr>,
    {
        ConfigBuilder::new(addr)
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn read_timeout(&self) -> Duration {
        self.read_timeout
    }

    pub fn write_timeout(&self) -> Duration {
        self.write_timeout
    }

    pub fn cache_enabled(&self) -> bool {
        self.cache_config.enable_cache
    }

    pub fn cache_ttl(&self) -> Option<Duration> {
        self.cache_config.ttl
    }

    pub fn cache_initial_capacity(&self) -> Option<usize> {
        self.cache_config.initial_capacity
    }

    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }
}

#[derive(Debug)]
pub struct ConfigBuilder {
    host: IpAddr,
    port: u16,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
    cache_config: CacheConfig,
    buffer_size: Option<usize>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct CacheConfig {
    pub(crate) enable_cache: bool,
    pub(crate) ttl: Option<Duration>,
    pub(crate) initial_capacity: Option<usize>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        CacheConfig {
            enable_cache: false,
            ttl: None,
            initial_capacity: None,
        }
    }
}

impl ConfigBuilder {
    pub fn new<A>(addr: A) -> ConfigBuilder
    where
        A: Into<IpAddr>,
    {
        ConfigBuilder {
            host: addr.into(),
            port: 9999,
            read_timeout: None,
            write_timeout: None,
            cache_config: Default::default(),
            buffer_size: None,
        }
    }

    pub fn with_port(&mut self, port: u16) -> &mut ConfigBuilder {
        self.port = port;
        self
    }

    pub fn with_read_timeout(&mut self, duration: Duration) -> &mut ConfigBuilder {
        self.read_timeout = Some(duration);
        self
    }

    pub fn with_write_timeout(&mut self, duration: Duration) -> &mut ConfigBuilder {
        self.write_timeout = Some(duration);
        self
    }

    pub fn with_cache_enabled(
        &mut self,
        ttl: Duration,
        initial_capacity: Option<usize>,
    ) -> &mut ConfigBuilder {
        self.cache_config = CacheConfig {
            enable_cache: true,
            ttl: Some(ttl),
            initial_capacity,
        };
        self
    }

    pub fn with_buffer_size(&mut self, buffer_size: usize) -> &mut ConfigBuilder {
        self.buffer_size = Some(buffer_size);
        self
    }

    pub fn build(&mut self) -> Config {
        let addr = SocketAddr::new(self.host, self.port);
        let read_timeout = self.read_timeout.unwrap_or(Duration::from_secs(3));
        let write_timeout = self.write_timeout.unwrap_or(Duration::from_secs(3));
        let buffer_size = self.buffer_size.unwrap_or(4096);
        let cache_config = self.cache_config;
        Config {
            addr,
            read_timeout,
            write_timeout,
            cache_config,
            buffer_size,
        }
    }
}
