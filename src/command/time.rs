use crate::error::Result;
use crate::proto::Proto;

use serde::{Deserialize, Serialize};
use std::fmt;

pub trait Time {
    fn time(&self) -> Result<DeviceTime>;
    fn timezone(&self) -> Result<DeviceTimeZone>;
}

pub(crate) struct TimeSetting {
    ns: String,
}

impl TimeSetting {
    pub(crate) fn new(ns: &str) -> Self {
        TimeSetting { ns: ns.into() }
    }

    pub(crate) fn get_time(&self, proto: &Proto) -> Result<DeviceTime> {
        proto
            .send_command(&self.ns, "get_time", None)
            .map(|mut res| serde_json::from_value(res[&self.ns]["get_time"].take()).unwrap())
    }

    pub(crate) fn get_timezone(&self, proto: &Proto) -> Result<DeviceTimeZone> {
        proto
            .send_command(&self.ns, "get_timezone", None)
            .map(|mut res| serde_json::from_value(res[&self.ns]["get_timezone"].take()).unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceTime {
    year: i32,
    month: u32,
    #[serde(alias = "mday")]
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
}

impl DeviceTime {
    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> u32 {
        self.month
    }

    pub fn day(&self) -> u32 {
        self.day
    }

    pub fn hour(&self) -> u32 {
        self.hour
    }

    pub fn minute(&self) -> u32 {
        self.min
    }

    pub fn second(&self) -> u32 {
        self.sec
    }
}

impl fmt::Display for DeviceTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.min, self.sec
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceTimeZone {
    index: i32,
}

impl DeviceTimeZone {
    pub fn index(&self) -> i32 {
        self.index
    }
}
