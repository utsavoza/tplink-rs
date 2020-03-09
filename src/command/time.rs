use crate::error::Result;
use crate::proto::Proto;

use serde::{Deserialize, Serialize};
use std::fmt;

/// The `Time` trait represents devices that are capable of maintaining
/// and providing their time and timezone.
pub trait Time {
    /// Attempts to fetch the device's time. Returns the current
    /// date and time of the device without the timezone.
    fn time(&self) -> Result<DeviceTime>;

    /// Attempts to fetch the device's timezone. Returns the current
    /// timezone of the device.
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
            .map(|mut res| {
                serde_json::from_value(res[&self.ns]["get_time"].take()).unwrap_or_else(|err| {
                    panic!(
                        "invalid response from host with address {}: {}",
                        proto.host(),
                        err
                    )
                })
            })
    }

    pub(crate) fn get_timezone(&self, proto: &Proto) -> Result<DeviceTimeZone> {
        proto
            .send_command(&self.ns, "get_timezone", None)
            .map(|mut res| {
                serde_json::from_value(res[&self.ns]["get_timezone"].take()).unwrap_or_else(|err| {
                    panic!(
                        "invalid response from host with address {}: {}",
                        proto.host(),
                        err
                    )
                })
            })
    }
}

/// The device's time without the timezone.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let plug = tplink::Plug::new([192, 168, 1, 100]);
///
/// let device_time = plug.time()?;
/// println!("{}", device_time);        // e.g. `2020-04-08 22:29:07`
///
/// let year = device_time.year();      // e.g. 2020
/// let month = device_time.month();    // e.g. 4
/// let day = device_time.day();        // e.g. 8
/// let hour = device_time.hour();      // e.g. 22
/// let minute = device_time.minute();  // e.g. 29
/// let second = device_time.second();  // e.g. 7
/// # Ok(())
/// # }
/// ```
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
    /// Returns the year number in the calendar date.
    pub fn year(&self) -> i32 {
        self.year
    }

    /// Returns the number starting from 1 to 12.
    pub fn month(&self) -> u32 {
        self.month
    }

    /// Returns the day of the month starting from 1.
    pub fn day(&self) -> u32 {
        self.day
    }

    /// Returns the hour number from 0 to 23.
    pub fn hour(&self) -> u32 {
        self.hour
    }

    /// Returns the minute number from 0 to 59.
    pub fn minute(&self) -> u32 {
        self.min
    }

    /// Returns the second number from 0 to 59.
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

/// The device's timezone.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceTimeZone {
    index: i32,
}

impl DeviceTimeZone {
    pub fn index(&self) -> i32 {
        self.index
    }
}
