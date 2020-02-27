use crate::crypto;
use crate::error::{self, Result};

use serde_json::Value;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::Duration;

pub(crate) struct Builder {
    host: IpAddr,
    port: u16,
    buffer_size: usize,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
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
            read_timeout: Some(Duration::from_secs(3)),
            write_timeout: Some(Duration::from_secs(3)),
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

    pub(crate) fn read_timeout(&mut self, read_timeout: Option<Duration>) -> &mut Builder {
        self.read_timeout = read_timeout;
        self
    }

    pub(crate) fn write_timeout(&mut self, write_timeout: Option<Duration>) -> &mut Builder {
        self.write_timeout = write_timeout;
        self
    }

    pub(crate) fn build(self) -> Proto {
        Proto {
            host: SocketAddr::new(self.host, self.port),
            buffer_size: self.buffer_size,
            read_timeout: self.read_timeout,
            write_timeout: self.write_timeout,
        }
    }
}

pub(crate) struct Proto {
    host: SocketAddr,
    buffer_size: usize,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
}

impl Proto {
    pub(crate) fn send_value(&self, value: &Value) -> Result<Value> {
        let bytes = serde_json::to_vec(value).unwrap();
        self.send_bytes(&crypto::encrypt(&bytes))
            .map(|res| serde_json::from_slice::<Value>(&res).map_err(error::json))?
    }

    pub(crate) fn send_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>> {
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
