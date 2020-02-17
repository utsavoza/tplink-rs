use crate::crypto;
use crate::error::Result;

use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::Duration;

#[derive(Debug)]
pub struct Plug {
    addr: SocketAddr,
}

impl Plug {
    pub fn new<A>(addr: A) -> Plug
    where
        A: Into<IpAddr>,
    {
        Plug {
            addr: SocketAddr::new(addr.into(), 9999),
        }
    }

    pub fn get_sys_info(&self) -> Result<String> {
        let socket = UdpSocket::bind("0.0.0.0:1234")?;
        socket.set_read_timeout(Some(Duration::from_secs(3)))?;
        socket.send_to(&system!({"get_sysinfo":{}}), self.addr)?;

        let mut buf = [0; 4096];
        match socket.recv(&mut buf) {
            Ok(recv) => {
                println!("read {} bytes", recv);
                let bytes = crypto::decrypt(&buf[..recv]);
                Ok(unsafe { String::from_utf8_unchecked(bytes) })
            }
            Err(e) => Err(e.into()),
        }
    }
}
