mod lb110;

use crate::bulb::lb110::{HSV, LB110};
use crate::command::{Device, SysInfo, System};
use crate::error::Result;

use std::net::IpAddr;
use std::time::Duration;

pub struct Bulb<T> {
    device: T,
}

impl<T: Device> Bulb<T> {
    pub fn turn_on(&mut self) -> Result<()> {
        self.device.turn_on()
    }

    pub fn turn_off(&mut self) -> Result<()> {
        self.device.turn_off()
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

impl<T: SysInfo> Bulb<T> {
    pub fn sysinfo(&self) -> Result<T::Info> {
        self.device.sysinfo()
    }
}

impl Bulb<LB110> {
    pub fn new<A>(host: A) -> Bulb<LB110>
    where
        A: Into<IpAddr>,
    {
        Bulb {
            device: LB110::new(host),
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

    pub fn rssi(&self) -> Result<i64> {
        self.device.rssi()
    }

    pub fn is_on(&self) -> Result<bool> {
        self.device.is_on()
    }

    pub fn hsv(&self) -> Result<HSV> {
        self.device.hsv()
    }

    pub fn has_emeter(&self) -> Result<bool> {
        Ok(true)
    }
}
