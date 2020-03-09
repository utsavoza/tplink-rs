use crate::command::sys::System;
use crate::command::sysinfo::SystemInfo;
use crate::command::time::{DeviceTime, DeviceTimeZone, TimeSetting};
use crate::command::{Device, Sys, SysInfo, Time};
use crate::error::Result;
use crate::proto::{self, Proto, Request};

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::fmt;
use std::net::IpAddr;
use std::time::Duration;

/// A TP-Link Wi-Fi Smart Plug (HS100).
pub struct HS100 {
    proto: Proto,
    system: System,
    time_setting: TimeSetting,
    sysinfo: SystemInfo<HS100Info>,
}

impl HS100 {
    pub(super) fn new<A>(host: A) -> HS100
    where
        A: Into<IpAddr>,
    {
        HS100 {
            proto: proto::Builder::default(host),
            system: System::new("system"),
            time_setting: TimeSetting::new("time"),
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
        self.sysinfo().map(|sysinfo| sysinfo.mac)
    }

    pub(super) fn rssi(&self) -> Result<i64> {
        self.sysinfo().map(|sysinfo| sysinfo.rssi)
    }

    pub(super) fn location(&self) -> Result<Location> {
        self.sysinfo().map(|sysinfo| sysinfo.location)
    }

    pub(super) fn is_on(&self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.is_on())
    }

    pub(super) fn is_led_on(&self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.is_led_on())
    }

    pub(super) fn turn_on_led(&mut self) -> Result<()> {
        self.proto.send_request(&Request::from(
            "system",
            "set_led_off",
            Some(json!({ "off": false })),
        ))?;
        Ok(())
    }

    pub(super) fn turn_off_led(&mut self) -> Result<()> {
        self.proto.send_request(&Request::from(
            "system",
            "set_led_off",
            Some(json!({ "off": true })),
        ))?;
        Ok(())
    }
}

impl Device for HS100 {
    fn turn_on(&mut self) -> Result<()> {
        self.proto.send_request(&Request::from(
            "system",
            "set_relay_state",
            Some(json!({ "state": 1 })),
        ))?;
        Ok(())
    }

    fn turn_off(&mut self) -> Result<()> {
        self.proto.send_request(&Request::from(
            "system",
            "set_relay_state",
            Some(json!({ "state": 0 })),
        ))?;
        Ok(())
    }
}

impl Sys for HS100 {
    fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.system.reboot(&self.proto, delay)
    }

    fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.system.factory_reset(&self.proto, delay)
    }
}

impl Time for HS100 {
    fn time(&self) -> Result<DeviceTime> {
        self.time_setting.get_time(&self.proto)
    }

    fn timezone(&self) -> Result<DeviceTimeZone> {
        self.time_setting.get_timezone(&self.proto)
    }
}

impl SysInfo for HS100 {
    type Info = HS100Info;

    fn sysinfo(&self) -> Result<Self::Info> {
        self.sysinfo.get_sysinfo(&self.proto)
    }
}

/// The system information of TP-Link Wi-Fi Smart Plug (HS100).
#[derive(Debug, Serialize, Deserialize)]
pub struct HS100Info {
    sw_ver: String,
    hw_ver: String,
    model: String,
    #[serde(rename = "type")]
    device_type: String,
    mac: String,
    alias: String,
    relay_state: u64,
    rssi: i64,
    #[serde(flatten)]
    location: Location,
    led_off: u64,
    #[serde(flatten)]
    other: Map<String, Value>,
}

/// The location coordinates of the device.
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "longitude_i")]
    pub longitude: i64,
    #[serde(rename = "latitude_i")]
    pub latitude: i64,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.latitude, self.longitude)
    }
}

impl HS100Info {
    /// Returns the software version of the device.
    pub fn sw_ver(&self) -> &str {
        &self.sw_ver
    }

    /// Returns the hardware version of the device.
    pub fn hw_ver(&self) -> &str {
        &self.hw_ver
    }

    /// Returns the model of the device.
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Returns the name (alias) of the device.
    pub fn alias(&self) -> &str {
        &self.alias
    }

    /// Returns the mac address of the device.
    pub fn mac_address(&self) -> &str {
        &self.mac
    }

    /// Returns the Wi-Fi signal strength (rssi) of the device.
    pub fn rssi(&self) -> i64 {
        self.rssi
    }

    /// Returns the location of the device.
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// Returns whether the device is on.
    fn is_on(&self) -> bool {
        self.relay_state == 1
    }

    /// Returns whether the device LED is on.
    fn is_led_on(&self) -> bool {
        self.led_off == 0
    }
}

impl fmt::Display for HS100Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
