use crate::error::Result;

use crate::proto::Proto;
use std::net::IpAddr;

#[derive(Debug)]
pub struct HS100 {
    proto: Proto,
}

impl HS100 {
    pub fn new<A>(host: A) -> HS100
    where
        A: Into<IpAddr>,
    {
        HS100 {
            proto: Proto::new(host.into()),
        }
    }
}

impl System for HS100 {
    fn sys_info(&self) -> Result<String> {
        self.proto
            .send(&system!({"get_sysinfo":{}}))
            .map(|res| unsafe { String::from_utf8_unchecked(res) })
    }

    fn turn_on(&self) -> Result<String> {
        self.proto
            .send(&system!({"set_relay_state":{"state":1}}))
            .map(|res| unsafe { String::from_utf8_unchecked(res) })
    }

    fn turn_off(&self) -> Result<String> {
        self.proto
            .send(&system!({"set_relay_state":{"state":0}}))
            .map(|res| unsafe { String::from_utf8_unchecked(res) })
    }
}

pub trait System {
    fn sys_info(&self) -> Result<String>;
    fn turn_on(&self) -> Result<String>;
    fn turn_off(&self) -> Result<String>;
}

pub struct Plug<T> {
    model: T,
}

impl Plug<HS100> {
    pub fn new<A>(addr: A) -> Plug<HS100>
    where
        A: Into<IpAddr>,
    {
        Plug {
            model: HS100::new(addr),
        }
    }
}

impl<T> Plug<T>
where
    T: System,
{
    pub fn sys_info(&self) -> Result<String> {
        self.model.sys_info()
    }

    pub fn turn_on(&self) -> Result<String> {
        self.model.turn_on()
    }

    pub fn turn_off(&self) -> Result<String> {
        self.model.turn_off()
    }
}
