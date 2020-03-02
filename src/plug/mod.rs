mod hs100;

use crate::command::{Device, System, SystemInfo};
use crate::error::Result;
use crate::plug::hs100::HS100;

use std::net::IpAddr;
use std::time::Duration;

pub struct Plug<T> {
    model: T,
}

impl Plug<HS100> {
    pub fn new<A>(host: A) -> Plug<HS100>
    where
        A: Into<IpAddr>,
    {
        Plug {
            model: HS100::new(host),
        }
    }
}

impl<T: SystemInfo> Plug<T> {
    pub fn sys_info(&mut self) -> Result<T::Info> {
        self.model.sys_info()
    }
}

impl<T: System> Plug<T> {
    pub fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.model.reboot(delay)
    }

    pub fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.model.factory_reset(delay)
    }
}

impl<T: Device> Plug<T> {
    pub fn turn_on(&mut self) -> Result<()> {
        self.model.turn_on()
    }

    pub fn turn_off(&mut self) -> Result<()> {
        self.model.turn_off()
    }
}
