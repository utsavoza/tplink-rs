mod hs100;

use crate::command::{Device, SysInfo, System};
use crate::error::Result;
use crate::plug::hs100::HS100;

use std::net::IpAddr;
use std::time::Duration;

pub struct Plug<T> {
    device: T,
}

impl Plug<HS100> {
    pub fn new<A>(host: A) -> Plug<HS100>
    where
        A: Into<IpAddr>,
    {
        Plug {
            device: HS100::new(host),
        }
    }
}

impl<T: SysInfo> Plug<T> {
    pub fn sysinfo(&mut self) -> Result<T::Info> {
        self.device.sysinfo()
    }
}

impl<T: System> Plug<T> {
    pub fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.device.reboot(delay)
    }

    pub fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.device.factory_reset(delay)
    }
}

impl<T: Device> Plug<T> {
    pub fn turn_on(&mut self) -> Result<()> {
        self.device.turn_on()
    }

    pub fn turn_off(&mut self) -> Result<()> {
        self.device.turn_off()
    }
}
