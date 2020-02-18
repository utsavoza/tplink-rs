use crate::crypto;
use crate::error::Result;

use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::Duration;

#[derive(Debug)]
pub struct Proto {
    addr: SocketAddr,
    buffer_size: Option<usize>,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
}

impl Proto {
    pub fn new<A>(host: A) -> Proto
    where
        A: Into<IpAddr>,
    {
        Proto {
            addr: SocketAddr::new(host.into(), 9999),
            buffer_size: Some(4096),
            read_timeout: Some(Duration::from_secs(3)),
            write_timeout: Some(Duration::from_secs(3)),
        }
    }

    pub fn send(&self, value: &serde_json::Value) -> Result<Vec<u8>> {
        let bytes = serde_json::to_vec(value).unwrap();
        self.send_to(&crypto::encrypt(&bytes))
    }

    fn send_to(&self, bytes: &[u8]) -> Result<Vec<u8>> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_read_timeout(self.read_timeout)?;
        socket.set_write_timeout(self.write_timeout)?;
        socket.send_to(bytes, self.addr)?;

        let mut buf = vec![0; self.buffer_size.unwrap()];
        match socket.recv(&mut buf) {
            Ok(recv) => Ok(crypto::decrypt(&buf[..recv])),
            Err(e) => Err(e.into()),
        }
    }
}
