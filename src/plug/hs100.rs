use crate::cache::Cache;
use crate::command::cloud::{Cloud, CloudInfo, CloudSettings};
use crate::command::device::Device;
use crate::command::sys::{Sys, System};
use crate::command::sysinfo::{SysInfo, SystemInfo};
use crate::command::time::{DeviceTime, DeviceTimeZone, Time, TimeSettings};
use crate::command::wlan::{AccessPoint, Netif, Wlan};
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
    time_setting: TimeSettings,
    cloud_setting: CloudSettings,
    netif: Netif,
    sysinfo: SystemInfo<HS100Info>,
    cache: Option<Cache<Request, Value>>,
}

impl HS100 {
    pub(super) fn new<A>(host: A) -> HS100
    where
        A: Into<IpAddr>,
    {
        HS100 {
            proto: proto::Builder::default(host),
            system: System::new("system"),
            time_setting: TimeSettings::new("time"),
            cloud_setting: CloudSettings::new("cnCloud"),
            netif: Netif::new(),
            sysinfo: SystemInfo::new(),
            cache: Some(Cache::with_ttl(Duration::from_secs(3))),
        }
    }

    pub(super) fn sw_ver(&mut self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.sw_ver)
    }

    pub(super) fn hw_ver(&mut self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.hw_ver)
    }

    pub(super) fn model(&mut self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.model)
    }

    pub(super) fn alias(&mut self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.alias)
    }

    pub(super) fn mac_address(&mut self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.mac)
    }

    pub(super) fn rssi(&mut self) -> Result<i64> {
        self.sysinfo().map(|sysinfo| sysinfo.rssi)
    }

    pub(super) fn location(&mut self) -> Result<Location> {
        self.sysinfo().map(|sysinfo| sysinfo.location)
    }

    pub(super) fn is_on(&mut self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.is_on())
    }

    pub(super) fn is_led_on(&mut self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.is_led_on())
    }

    pub(super) fn turn_on_led(&mut self) -> Result<()> {
        if let Some(c) = self.cache.as_mut() {
            c.retain(|k, _| k.target != "system");
        }

        let response = self.proto.send_request(&Request::new(
            "system",
            "set_led_off",
            Some(json!({ "off": false })),
        ))?;

        log::trace!("(system) {:?}", response);

        Ok(())
    }

    pub(super) fn turn_off_led(&mut self) -> Result<()> {
        if let Some(c) = self.cache.as_mut() {
            c.retain(|k, _| k.target != "system");
        }

        let response = self.proto.send_request(&Request::new(
            "system",
            "set_led_off",
            Some(json!({ "off": true })),
        ))?;

        log::trace!("(system) {:?}", response);

        Ok(())
    }
}

impl Device for HS100 {
    fn turn_on(&mut self) -> Result<()> {
        if let Some(c) = self.cache.as_mut() {
            c.retain(|k, _| k.target != "system");
        }

        let response = self.proto.send_request(&Request::new(
            "system",
            "set_relay_state",
            Some(json!({ "state": 1 })),
        ))?;

        log::trace!("(system) {:?}", response);

        Ok(())
    }

    fn turn_off(&mut self) -> Result<()> {
        if let Some(c) = self.cache.as_mut() {
            c.retain(|k, _| k.target != "system");
        }

        let response = self.proto.send_request(&Request::new(
            "system",
            "set_relay_state",
            Some(json!({ "state": 0 })),
        ))?;

        log::trace!("(system) {:?}", response);

        Ok(())
    }
}

impl Sys for HS100 {
    fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.system.reboot(&self.proto, self.cache.as_mut(), delay)
    }

    fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.system.reset(&self.proto, self.cache.as_mut(), delay)
    }
}

impl Time for HS100 {
    fn time(&mut self) -> Result<DeviceTime> {
        self.time_setting.get_time(&self.proto)
    }

    fn timezone(&mut self) -> Result<DeviceTimeZone> {
        self.time_setting.get_timezone(&self.proto)
    }
}

impl Cloud for HS100 {
    fn get_cloud_info(&mut self) -> Result<CloudInfo> {
        self.cloud_setting
            .get_info(&self.proto, self.cache.as_mut())
    }

    fn bind(&mut self, username: &str, password: &str) -> Result<()> {
        self.cloud_setting
            .bind(&self.proto, self.cache.as_mut(), username, password)
    }

    fn unbind(&mut self) -> Result<()> {
        self.cloud_setting.unbind(&self.proto, self.cache.as_mut())
    }

    fn get_firmware_list(&mut self) -> Result<Vec<String>> {
        self.cloud_setting
            .get_firmware_list(&self.proto, self.cache.as_mut())
    }

    fn set_server_url(&mut self, url: &str) -> Result<()> {
        self.cloud_setting
            .set_server_url(&self.proto, self.cache.as_mut(), url)
    }
}

impl Wlan for HS100 {
    fn get_scan_info(
        &mut self,
        refresh: bool,
        timeout: Option<Duration>,
    ) -> Result<Vec<AccessPoint>> {
        self.netif.get_scan_info(&self.proto, refresh, timeout)
    }
}

impl SysInfo for HS100 {
    type Info = HS100Info;

    fn sysinfo(&mut self) -> Result<Self::Info> {
        self.sysinfo.get_sysinfo(&self.proto, self.cache.as_mut())
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
