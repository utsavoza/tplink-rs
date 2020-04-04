use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

/// Configuration options used to configure a TP-Link device.
///
/// The configuration consists of options that define the protocol that
/// device instances use in order to communicate with the host devices
/// over the local network.
///
/// # Examples
///
/// ```
/// use std::net::IpAddr;
/// use std::time::Duration;
///
/// // Define the configuration for the device.
/// let config = tplink::Config::for_host([192, 168, 1, 100])
///     .with_cache_enabled(Duration::from_secs(3), None)
///     .build();
///
/// assert_eq!(config.addr(), IpAddr::from([192, 168, 1, 100]));
/// assert_eq!(config.cache_enabled(), true);
/// assert_eq!(config.cache_ttl(), Some(Duration::from_secs(3)));
///
/// // Create a new plug instance with the config.
/// let plug = tplink::Plug::with_config(config);
/// ```
#[derive(Debug)]
pub struct Config {
    pub(crate) addr: SocketAddr,
    pub(crate) read_timeout: Duration,
    pub(crate) write_timeout: Duration,
    pub(crate) cache_config: CacheConfig,
    pub(crate) buffer_size: usize,
}

impl Config {
    /// Returns a new configuration [`Builder`] for the given local address
    /// of the host device with all the default configurations specified.
    ///
    /// [`Builder`]: struct.Builder.html
    ///
    /// # Examples
    ///
    /// ```
    /// let config = tplink::Config::for_host([192, 168, 1, 100]).build();
    /// ```
    pub fn for_host<A>(addr: A) -> ConfigBuilder
    where
        A: Into<IpAddr>,
    {
        ConfigBuilder::new(addr)
    }

    /// Returns the configured local address of host device.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::IpAddr;
    ///
    /// let config = tplink::Config::for_host([192, 168, 1, 100]).build();
    /// assert_eq!(config.addr(), IpAddr::from([192, 168, 1, 100]));
    /// ```
    pub fn addr(&self) -> IpAddr {
        self.addr.ip()
    }

    /// Returns the configured port number associated with the device's
    /// host address.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = tplink::Config::for_host([192, 168, 1, 100])
    ///     .with_port(9999)
    ///     .build();
    /// assert_eq!(config.port(), 9999);
    /// ```
    pub fn port(&self) -> u16 {
        self.addr.port()
    }

    /// Returns the configured read timeout for the device.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let config = tplink::Config::for_host([192, 168, 1, 100])
    ///     .with_read_timeout(Duration::from_secs(5))
    ///     .build();
    /// assert_eq!(config.read_timeout(), Duration::from_secs(5));
    /// ```
    pub fn read_timeout(&self) -> Duration {
        self.read_timeout
    }

    /// Returns the configured write timeout for the device.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let config = tplink::Config::for_host([192, 168, 1, 100])
    ///     .with_write_timeout(Duration::from_secs(5))
    ///     .build();
    /// assert_eq!(config.write_timeout(), Duration::from_secs(5));
    /// ```
    pub fn write_timeout(&self) -> Duration {
        self.write_timeout
    }

    /// Returns true if caching is enabled for the device, and false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let config = tplink::Config::for_host([192, 168, 1, 100])
    ///     .with_cache_enabled(Duration::from_secs(3), None)
    ///     .build();
    /// assert_eq!(config.cache_enabled(), true);
    /// ```
    pub fn cache_enabled(&self) -> bool {
        self.cache_config.enable_cache
    }

    /// Returns the configured cache ttl (time-to-live) for the device if
    /// caching is enabled, and `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let config = tplink::Config::for_host([192, 168, 1, 100])
    ///     .with_cache_enabled(Duration::from_secs(3), None)
    ///     .build();
    /// assert_eq!(config.cache_ttl(), Some(Duration::from_secs(3)));
    /// ```
    pub fn cache_ttl(&self) -> Option<Duration> {
        self.cache_config.ttl
    }

    /// Returns the configured initial capacity for the cache if caching is
    /// enabled, and None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let config = tplink::Config::for_host([192, 168, 1, 100])
    ///     .with_cache_enabled(Duration::from_secs(3), Some(1024))
    ///     .build();
    /// assert_eq!(config.cache_initial_capacity(), Some(1024));
    /// ```
    pub fn cache_initial_capacity(&self) -> Option<usize> {
        self.cache_config.initial_capacity
    }

    /// Returns the configured response buffer size for the device.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// let config = tplink::Config::for_host([192, 168, 1, 100])
    ///     .with_buffer_size(4096)
    ///     .build();
    /// assert_eq!(config.buffer_size(), 4096);
    /// ```
    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }
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

/// Builds TP-Link device [`Config`] instance with custom configuration values.
///
/// Methods can be chained in order to set the configuration values. The [`Config`]
/// instance is constructed by calling [`build`].
///
/// New instances of the `ConfigBuilder` are obtained via [`Config::for_host`].
///
/// See function level documentation for details on various configuration
/// settings.
///
/// [`Config`]: ./struct.Config.html
/// [`Config::for_host`]: ./struct.Config.html#method.for_host
/// [`build`]: #method.build
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// // Define the configuration for the device.
/// let config = tplink::Config::for_host([192, 168, 1, 100])
///     .with_port(9999)
///     .with_read_timeout(Duration::from_secs(5))
///     .with_write_timeout(Duration::from_secs(5))
///     .with_cache_enabled(Duration::from_secs(3), None)
///     .with_buffer_size(4 * 1024)
///     .build();
///
/// // Create a new bulb instance with the config.
/// let bulb = tplink::Bulb::with_config(config);
/// ```
#[derive(Debug)]
pub struct ConfigBuilder {
    host: IpAddr,
    port: u16,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
    cache_config: CacheConfig,
    buffer_size: Option<usize>,
}

impl ConfigBuilder {
    /// Returns a new builder for the given local address of the host device
    /// with all the default configurations specified.
    fn new<A>(addr: A) -> ConfigBuilder
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

    /// Sets the port number associated with the device's host address.
    ///
    /// The default port used is 9999.
    ///
    /// It is advised to use the default port and not set the property manually
    /// as all the devices currently respond on default port only.
    pub fn with_port(&mut self, port: u16) -> &mut ConfigBuilder {
        self.port = port;
        self
    }

    /// Sets the read timeout to the specified timeout duration.
    ///
    /// If not set, then the default read timeout used is 3 seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let config = tplink::Config::for_host([192, 168, 1, 100])
    ///     .with_read_timeout(Duration::from_secs(5))
    ///     .build();
    /// assert_eq!(config.read_timeout(), Duration::from_secs(5));
    /// ```
    pub fn with_read_timeout(&mut self, duration: Duration) -> &mut ConfigBuilder {
        self.read_timeout = Some(duration);
        self
    }

    /// Sets the write timeout to the specified timeout duration.
    ///
    /// If not set, then the default write timeout used is 3 seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let config = tplink::Config::for_host([192, 168, 1, 100])
    ///     .with_write_timeout(Duration::from_secs(5))
    ///     .build();
    /// assert_eq!(config.write_timeout(), Duration::from_secs(5));
    /// ```
    pub fn with_write_timeout(&mut self, duration: Duration) -> &mut ConfigBuilder {
        self.write_timeout = Some(duration);
        self
    }

    /// Enables caching device responses with the specified cache ttl (time-to-live)
    /// and initial cache capacity.
    ///
    /// By default, caching is disabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let config = tplink::Config::for_host([192, 168, 1, 100])
    ///     .with_cache_enabled(Duration::from_secs(3), Some(2 * 1024))
    ///     .build();
    /// assert_eq!(config.cache_enabled(), true);
    /// assert_eq!(config.cache_ttl(), Some(Duration::from_secs(3)));
    /// assert_eq!(config.cache_initial_capacity(), Some(2 * 1024));
    /// ```
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

    /// Sets the device's response buffer size.
    ///
    /// The buffer size should be large enough to hold device's response bytes. If the
    /// response is too long to fit in the buffer of specified size, excess bytes maybe
    /// discarded and the device would eventually return an [`Error`] on every request.
    ///
    /// The default value used is 4096.
    ///
    /// [`Error`]: ../struct.Error.html
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// let config = tplink::Config::for_host([192, 168, 1, 100])
    ///     .with_buffer_size(5 * 1024)
    ///     .build();
    /// assert_eq!(config.buffer_size(), 5 * 1024);
    /// ```
    pub fn with_buffer_size(&mut self, buffer_size: usize) -> &mut ConfigBuilder {
        self.buffer_size = Some(buffer_size);
        self
    }

    /// Creates a new configured [`Config`] instance.
    ///
    /// [`Config`]: struct.Config.html
    /// # Examples
    ///
    /// ```
    ///
    /// // Create a new config.
    /// let config = tplink::Config::for_host([192, 168, 1, 100]).build();
    ///
    /// // Use the config to create a new device instance.
    /// let plug = tplink::Plug::with_config(config);
    /// ```
    pub fn build(&mut self) -> Config {
        let addr = SocketAddr::new(self.host, self.port);
        let cache_config = self.cache_config;

        // Set the default read timeout to 3 seconds
        let read_timeout = self.read_timeout.unwrap_or(Duration::from_secs(3));

        // Set the default write timeout to 3 seconds
        let write_timeout = self.write_timeout.unwrap_or(Duration::from_secs(3));

        // Set the default buffer size to 4 * 1024
        let buffer_size = self.buffer_size.unwrap_or(4 * 1024);

        Config {
            addr,
            read_timeout,
            write_timeout,
            cache_config,
            buffer_size,
        }
    }
}
