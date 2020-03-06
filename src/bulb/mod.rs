mod lb1xx;

use crate::bulb::lb1xx::LB1XX;
use crate::command::{Device, SysInfo, System};
use crate::error::Result;

use std::net::IpAddr;
use std::time::Duration;

pub struct Bulb<T> {
    device: T,
}

impl<T: SysInfo> Bulb<T> {
    pub fn sysinfo(&self) -> Result<T::Info> {
        self.device.sysinfo()
    }
}

impl<T: System> Bulb<T> {
    pub fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.device.reboot(delay)
    }

    pub fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.device.factory_reset(delay)
    }
}

impl<T: Device> Bulb<T> {
    pub fn turn_on(&mut self) -> Result<()> {
        self.device.turn_on()
    }

    pub fn turn_off(&mut self) -> Result<()> {
        self.device.turn_off()
    }
}

impl Bulb<LB1XX> {
    pub fn new<A>(host: A) -> Bulb<LB1XX>
    where
        A: Into<IpAddr>,
    {
        Bulb {
            device: LB1XX::new(host),
        }
    }

    pub fn sw_ver(&self) -> Result<String> {
        self.device.sw_ver()
    }

    pub fn hw_ver(&self) -> Result<String> {
        self.device.hw_ver()
    }

    pub fn model(&self) -> Result<String> {
        self.device.model()
    }

    pub fn alias(&self) -> Result<String> {
        self.device.alias()
    }

    pub fn mac_address(&self) -> Result<String> {
        self.device.mac_address()
    }

    pub fn is_dimmable(&self) -> Result<bool> {
        self.device.is_dimmable()
    }

    pub fn is_color(&self) -> Result<bool> {
        self.device.is_color()
    }

    pub fn is_variable_color_temp(&self) -> Result<bool> {
        self.device.is_variable_color_temp()
    }

    pub fn is_on(&self) -> Result<bool> {
        self.device.is_on()
    }

    pub fn hsv(&self) -> Result<(u64, u64, u64)> {
        self.device.hsv()
    }
}
