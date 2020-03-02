mod lb110;
mod lighting;

use crate::bulb::lb110::LB110;
use crate::command::{Device, System, SystemInfo};
use crate::error::Result;

use std::net::IpAddr;
use std::time::Duration;

pub struct Bulb<T> {
    model: T,
}

impl Bulb<LB110> {
    pub fn new<A>(host: A) -> Bulb<LB110>
    where
        A: Into<IpAddr>,
    {
        Bulb {
            model: LB110::new(host),
        }
    }
}

impl<T: SystemInfo> Bulb<T> {
    pub fn sys_info(&mut self) -> Result<T::Info> {
        self.model.sys_info()
    }
}

impl<T: System> Bulb<T> {
    pub fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.model.reboot(delay)
    }

    pub fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.model.factory_reset(delay)
    }
}

impl<T: Device> Bulb<T> {
    pub fn turn_on(&mut self) -> Result<()> {
        self.model.turn_on()
    }

    pub fn turn_off(&mut self) -> Result<()> {
        self.model.turn_off()
    }
}
