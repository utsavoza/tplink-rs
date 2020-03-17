use crate::crypto;
use crate::error::{self, Result};

use serde_json::{json, Value};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::ErrorKind;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::Duration;

pub struct Request {
    pub target: String,
    pub command: String,
    pub arg: Option<Value>,
}

impl Request {
    pub fn new(target: &str, command: &str, arg: Option<Value>) -> Request {
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

pub struct Builder {
    host: IpAddr,
    port: u16,
    buffer_size: usize,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
    broadcast: bool,
    tolerance: u32,
}

impl Builder {
    pub fn new<A>(host: A) -> Builder
    where
        A: Into<IpAddr>,
    {
        Builder {
            host: host.into(),
            port: 9999,
            buffer_size: 4096,
            read_timeout: None,
            write_timeout: None,
            broadcast: false,
            tolerance: 1,
        }
    }

    pub fn default<A>(host: A) -> Proto
    where
        A: Into<IpAddr>,
    {
        Self::new(host)
            .port(9999)
            .buffer_size(4096)
            .read_timeout(Duration::from_secs(3))
            .write_timeout(Duration::from_secs(3))
            .broadcast(false)
            .build()
    }

    pub fn port(&mut self, port: u16) -> &mut Builder {
        self.port = port;
        self
    }

    pub fn buffer_size(&mut self, buffer_size: usize) -> &mut Builder {
        self.buffer_size = buffer_size;
        self
    }

    pub fn read_timeout(&mut self, duration: Duration) -> &mut Builder {
        self.read_timeout = Some(duration);
        self
    }

    pub fn write_timeout(&mut self, duration: Duration) -> &mut Builder {
        self.write_timeout = Some(duration);
        self
    }

    pub fn broadcast(&mut self, broadcast: bool) -> &mut Builder {
        self.broadcast = broadcast;
        self
    }

    pub fn tolerance(&mut self, offline_tolerance: u32) -> &mut Builder {
        self.tolerance = offline_tolerance;
        self
    }

    pub fn build(&mut self) -> Proto {
        Proto {
            addr: SocketAddr::new(self.host, self.port),
            buffer_size: self.buffer_size,
            read_timeout: self.read_timeout,
            write_timeout: self.write_timeout,
            broadcast: self.broadcast,
            tolerance: self.tolerance,
        }
    }
}

pub struct Proto {
    addr: SocketAddr,
    buffer_size: usize,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
    broadcast: bool,
    tolerance: u32,
}

impl Proto {
    pub fn host(&self) -> IpAddr {
        self.addr.ip()
    }

    pub fn read_timeout(&self) -> Option<Duration> {
        self.read_timeout
    }

    pub fn discover(&self, req: &[u8]) -> Result<HashMap<IpAddr, Vec<u8>>> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        socket.set_broadcast(self.broadcast)?;
        socket.set_read_timeout(self.read_timeout)?;
        socket.set_write_timeout(self.write_timeout)?;

        for _ in 0..self.tolerance {
            socket.send_to(&crypto::encrypt(req), &self.addr)?;
        }

        let mut responses = HashMap::new();
        let mut buf = vec![0; self.buffer_size];
        loop {
            match socket.recv_from(&mut buf) {
                Ok((recv, addr)) => {
                    responses
                        .entry(addr.ip())
                        .or_insert_with(|| crypto::decrypt(&buf[..recv]));
                }
                Err(e) => {
                    return if e.kind() == ErrorKind::WouldBlock {
                        Ok(responses)
                    } else {
                        Err(e.into())
                    }
                }
            }
        }
    }

    pub fn send_request(&self, req: &Request) -> Result<Value> {
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

        socket.set_broadcast(self.broadcast)?;
        socket.set_read_timeout(self.read_timeout)?;
        socket.set_write_timeout(self.write_timeout)?;

        for _ in 0..self.tolerance {
            socket.send_to(&crypto::encrypt(req), self.addr)?;
        }

        let mut buf = vec![0; self.buffer_size];
        match socket.recv(&mut buf) {
            Ok(recv) => Ok(crypto::decrypt(&buf[..recv])),
            Err(e) => Err(e.into()),
        }
    }
}
