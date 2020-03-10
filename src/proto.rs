use crate::crypto;
use crate::error::{self, Result};

use serde_json::{json, Value};
use std::hash::{Hash, Hasher};
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::Duration;

#[derive(Debug)]
pub(crate) struct Request {
    pub(crate) target: String,
    pub(crate) command: String,
    pub(crate) arg: Option<Value>,
}

impl Request {
    pub(crate) fn new(target: &str, command: &str, arg: Option<Value>) -> Request {
        Request {
            target: target.into(),
            command: command.into(),
            arg,
        }
    }
}

impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target && self.command == other.command
    }
}

impl Eq for Request {}

impl Hash for Request {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.target.hash(state);
        self.command.hash(state);
    }
}

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
            read_timeout: None,
            write_timeout: None,
        }
    }

    pub(crate) fn default<A>(host: A) -> Proto
    where
        A: Into<IpAddr>,
    {
        Self::new(host)
            .port(9999)
            .buffer_size(4096)
            .read_timeout(Duration::from_secs(3))
            .write_timeout(Duration::from_secs(3))
            .build()
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

    pub(crate) fn build(&mut self) -> Proto {
        Proto {
            addr: SocketAddr::new(self.host, self.port),
            buffer_size: self.buffer_size,
            read_timeout: self.read_timeout,
            write_timeout: self.write_timeout,
        }
    }
}

pub(crate) struct Proto {
    addr: SocketAddr,
    buffer_size: usize,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
}

impl Proto {
    pub(crate) fn host(&self) -> IpAddr {
        self.addr.ip()
    }

    pub(crate) fn send_request(&self, req: &Request) -> Result<Value> {
        let Request {
            target,
            command,
            arg,
        } = req;
        serde_json::to_vec(&json!({ target: { command: arg } }))
            .map_err(error::json)
            .and_then(|req| self.send_bytes(&req))
            .and_then(|res| {
                serde_json::from_slice::<Value>(&res)
                    .map(|mut value| value[target][command].take())
                    .map_err(error::json)
            })
    }

    fn send_bytes(&self, req: &[u8]) -> Result<Vec<u8>> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        socket.set_read_timeout(self.read_timeout)?;
        socket.set_write_timeout(self.write_timeout)?;
        socket.send_to(&crypto::encrypt(req), self.addr)?;

        let mut buf = vec![0; self.buffer_size];
        match socket.recv(&mut buf) {
            Ok(recv) => Ok(crypto::decrypt(&buf[..recv])),
            Err(e) => Err(e.into()),
        }
    }
}
