mod info;
mod lighting;
mod system;

use crate::bulb::info::{Info, LB110Info};
use crate::bulb::lighting::Lighting;
use crate::bulb::system::System as Sys;
use crate::command::{Device, System, SystemInfo};
use crate::error::Result;
use crate::proto::{self, Proto};

use log::debug;
use serde_json::json;
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
    pub fn sysinfo(&mut self) -> Result<T::Info> {
        self.model.sysinfo()
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

pub struct LB110 {
    proto: Proto,
    sys: Sys,
    info: Info,
    lighting: Lighting,
}

impl LB110 {
    fn new<A>(host: A) -> LB110
    where
        A: Into<IpAddr>,
    {
        LB110 {
            proto: proto::Builder::default(host),
            lighting: Lighting::new(None),
            info: Info::new(None),
            sys: Sys::new(None),
        }
    }
}

impl SystemInfo for LB110 {
    type Info = LB110Info;

    fn sysinfo(&mut self) -> Result<Self::Info> {
        self.info.get_sysinfo(&mut self.proto)
    }
}

impl System for LB110 {
    fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.sys.reboot(
            &mut self.proto,
            Some(&json!({ "delay": delay.map_or(1, |duration| duration.as_secs()) })),
        )
    }

    fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.sys.reset(
            &mut self.proto,
            Some(&json!({ "delay": delay.map_or(1, |duration| duration.as_secs()) })),
        )
    }
}

impl Device for LB110 {
    fn turn_on(&mut self) -> Result<()> {
        self.lighting
            .set_light_state(&mut self.proto, Some(&json!({ "on_off": 1 })))
            .map(|state| debug!("{:?}", state))
    }

    fn turn_off(&mut self) -> Result<()> {
        self.lighting
            .set_light_state(&mut self.proto, Some(&json!({ "on_off": 0 })))
            .map(|state| debug!("{:?}", state))
    }
}
