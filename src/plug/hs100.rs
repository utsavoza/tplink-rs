use crate::command::{Device, SysInfo, System};
use crate::error::Result;
use crate::proto::{self, Proto};

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::fmt;
use std::net::IpAddr;
use std::time::Duration;

pub struct HS100 {
    proto: Proto,
}

impl HS100 {
    pub(super) fn new<A>(host: A) -> HS100
    where
        A: Into<IpAddr>,
    {
        HS100 {
            proto: proto::Builder::default(host),
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
        self.sysinfo()
            .map(|sysinfo| sysinfo.mac)
    }

    pub(super) fn rssi(&self) -> Result<i64> {
        self.sysinfo().map(|sysinfo| sysinfo.rssi)
    }

    pub(super) fn location(&self) -> Result<Location> {
        self.sysinfo().map(|sysinfo| sysinfo.location)
    }

    pub(super) fn is_on(&self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.relay_state() == 1)
    }
}

impl Device for HS100 {
    fn turn_on(&mut self) -> Result<()> {
        self.proto
            .send("system", "set_relay_state", Some(&json!({ "state": 1 })))
            .map(|_| {})
    }

    fn turn_off(&mut self) -> Result<()> {
        self.proto
            .send("system", "set_relay_state", Some(&json!({ "state": 0 })))
            .map(|_| {})
    }
}

impl System for HS100 {
    fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        let delay_in_secs = delay.map_or(1, |duration| duration.as_secs());
        self.proto
            .send("system", "reboot", Some(&json!({ "delay": delay_in_secs })))
            .map(|_| {})
    }

    fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        let delay_in_secs = delay.map_or(1, |duration| duration.as_secs());
        self.proto
            .send("system", "reset", Some(&json!({ "delay": delay_in_secs })))
            .map(|_| {})
    }
}

impl SysInfo for HS100 {
    type Info = HS100Info;

    fn sysinfo(&self) -> Result<Self::Info> {
        self.proto.send("system", "get_sysinfo", None).map(|res| {
            serde_json::from_slice::<Response>(&res)
                .map(|res| res.system.get_sysinfo)
                .unwrap()
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    #[serde(alias = "system")]
    system: GetSysInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetSysInfo {
    get_sysinfo: HS100Info,
}

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
    #[serde(flatten)]
    other: Map<String, Value>,
}

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
        &self.mac
    }

    pub fn rssi(&self) -> i64 {
        self.rssi
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    fn relay_state(&self) -> u64 {
        self.relay_state
    }
}

impl fmt::Display for HS100Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
