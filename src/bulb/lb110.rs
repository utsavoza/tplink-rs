use crate::bulb::lighting::{LightState, Lighting, HSV};
use crate::command::system::System;
use crate::command::time::{DeviceTime, DeviceTimeZone, TimeSetting};
use crate::command::{Device, SysInfo, Sys, Time};
use crate::error::{self, Result};
use crate::proto::{self, Proto};

use crate::command::sysinfo::SystemInfo;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::fmt;
use std::net::IpAddr;
use std::time::Duration;

pub struct LB110 {
    proto: Proto,
    system: System,
    lighting: Lighting,
    time_setting: TimeSetting,
    sysinfo: SystemInfo<LB110Info>,
}

impl LB110 {
    pub(super) fn new<A>(host: A) -> LB110
    where
        A: Into<IpAddr>,
    {
        LB110 {
            proto: proto::Builder::default(host),
            system: System::new("smartlife.iot.common.system"),
            lighting: Lighting::new("smartlife.iot.smartbulb.lightingservice"),
            time_setting: TimeSetting::new("smartlife.iot.common.timesetting"),
            sysinfo: SystemInfo::new(),
        }
    }

    pub(super) fn sw_ver(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.sw_ver)
    }

    pub(super) fn hw_ver(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.hw_ver)
    }

    pub(super) fn model(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.model)
    }

    pub(super) fn alias(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.alias)
    }

    pub(super) fn mac_address(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.mic_mac)
    }

    pub(super) fn is_dimmable(&self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.is_dimmable())
    }

    pub(super) fn is_color(&self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.is_color())
    }

    pub(super) fn is_variable_color_temp(&self) -> Result<bool> {
        self.sysinfo()
            .map(|sysinfo| sysinfo.is_variable_color_temp())
    }

    pub(super) fn rssi(&self) -> Result<i64> {
        self.sysinfo().map(|sysinfo| sysinfo.rssi())
    }

    pub(super) fn hsv(&self) -> Result<HSV> {
        self.sysinfo().and_then(|sysinfo| sysinfo.hsv())
    }

    pub(super) fn is_on(&self) -> Result<bool> {
        self.lighting
            .get_light_state(&self.proto)
            .map(|light_state| light_state.is_on())
    }
}

impl Device for LB110 {
    fn turn_on(&mut self) -> Result<()> {
        self.lighting
            .set_light_state(&self.proto, Some(&json!({ "on_off": 1 })))
            .map(|state| log::trace!("{:?}", state))
    }

    fn turn_off(&mut self) -> Result<()> {
        self.lighting
            .set_light_state(&self.proto, Some(&json!({ "on_off": 0 })))
            .map(|state| log::trace!("{:?}", state))
    }
}

impl Sys for LB110 {
    fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.system.reboot(&self.proto, delay)
    }

    fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.system.factory_reset(&self.proto, delay)
    }
}

impl Time for LB110 {
    fn time(&self) -> Result<DeviceTime> {
        self.time_setting.get_time(&self.proto)
    }

    fn timezone(&self) -> Result<DeviceTimeZone> {
        self.time_setting.get_timezone(&self.proto)
    }
}

impl SysInfo for LB110 {
    type Info = LB110Info;

    fn sysinfo(&self) -> Result<Self::Info> {
        self.sysinfo.get_sysinfo(&self.proto)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    #[serde(alias = "system")]
    system: Option<GetSysInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetSysInfo {
    get_sysinfo: LB110Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LB110Info {
    sw_ver: String,
    hw_ver: String,
    model: String,
    description: Option<String>,
    alias: String,
    mic_type: String,
    mic_mac: String,
    is_dimmable: u64,
    is_color: u64,
    is_variable_color_temp: u64,
    light_state: LightState,
    rssi: i64,
    #[serde(flatten)]
    other: Map<String, Value>,
}

impl LB110Info {
    pub fn sw_ver(&self) -> &str {
        &self.sw_ver
    }

    pub fn hw_ver(&self) -> &str {
        &self.hw_ver
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn alias(&self) -> &str {
        &self.alias
    }

    pub fn mac_address(&self) -> &str {
        &self.mic_mac
    }

    pub fn is_dimmable(&self) -> bool {
        self.is_dimmable == 1
    }

    pub fn is_color(&self) -> bool {
        self.is_color == 1
    }

    pub fn is_variable_color_temp(&self) -> bool {
        self.is_variable_color_temp == 1
    }

    pub fn rssi(&self) -> i64 {
        self.rssi
    }

    pub fn hsv(&self) -> Result<HSV> {
        if self.is_color == 1 {
            Ok(self.light_state.hsv())
        } else {
            Err(error::unsupported_operation("hsv"))
        }
    }
}

impl fmt::Display for LB110Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
