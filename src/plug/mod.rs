mod hs100;

use crate::command::time::{DeviceTime, DeviceTimeZone};
use crate::command::{Device, Sys, SysInfo, Time};
use crate::error::Result;
use crate::plug::hs100::Location;
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

    pub fn rssi(&self) -> Result<i64> {
        self.device.rssi()
    }

    pub fn location(&self) -> Result<Location> {
        self.device.location()
    }

    pub fn is_on(&self) -> Result<bool> {
        self.device.is_on()
    }
}

impl<T: SysInfo> Plug<T> {
    pub fn sysinfo(&self) -> Result<T::Info> {
        self.device.sysinfo()
    }
}

impl<T: Sys> Plug<T> {
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

impl<T: Time> Plug<T> {
    pub fn time(&self) -> Result<DeviceTime> {
        self.device.time()
    }

    pub fn timezone(&self) -> Result<DeviceTimeZone> {
        self.device.timezone()
    }
}
