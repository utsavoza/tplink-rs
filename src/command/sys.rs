use crate::error::Result;
use crate::proto::{Proto, Request};

use serde_json::{json, Value};
use std::time::Duration;

/// The `Sys` trait represents devices that are capable of performing
/// system commands.
pub trait Sys {
    /// Reboots the device after the given duration. In case when the duration
    /// isn't provided, the device is set to reboot after a default duration
    /// of 1 second.
    fn reboot(&mut self, delay: Option<Duration>) -> Result<()>;

    /// Factory resets the device after the given duration. In case when the
    /// duration isn't provided, the device is set to reset after a default duration
    /// of 1 second.
    fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()>;
}

pub(crate) struct System {
    ns: String,
}

impl System {
    pub(crate) fn new(ns: &str) -> System {
        System { ns: ns.into() }
    }

    pub(crate) fn reboot(&self, proto: &Proto, delay: Option<Duration>) -> Result<Value> {
        let delay_in_secs = delay.map_or(1, |duration| duration.as_secs());
        proto.send_request(&Request::new(
            &self.ns,
            "reboot",
            Some(json!({ "delay": delay_in_secs })),
        ))
    }

    pub(crate) fn factory_reset(&self, proto: &Proto, delay: Option<Duration>) -> Result<Value> {
        let delay_in_secs = delay.map_or(1, |duration| duration.as_secs());
        proto.send_request(&Request::new(
            &self.ns,
            "reset",
            Some(json!({ "delay": delay_in_secs })),
        ))
    }
}
