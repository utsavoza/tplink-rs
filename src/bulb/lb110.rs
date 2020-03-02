use crate::bulb::lighting::Lighting;
use crate::command::{Device, System};
use crate::error::Result;
use crate::proto::{self, Proto};

use log::{debug, error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt;
use std::net::IpAddr;
use std::time::Duration;

pub struct LB110 {
    proto: Proto,
    lighting: Lighting,
}

impl LB110 {
    pub(super) fn new<A>(host: A) -> LB110
    where
        A: Into<IpAddr>,
    {
        LB110 {
            proto: proto::Builder::default(host),
            lighting: Lighting::new(None),
        }
    }
}

impl System for LB110 {
    type SystemInfo = LB110Info;

    fn sys_info(&mut self) -> Result<Self::SystemInfo> {
        self.proto.send("system", "get_sysinfo", None).map(|res| {
            match serde_json::from_slice::<Response>(&res) {
                Ok(res) => {
                    debug!("{}", res);
                    res.sys_info
                }
                Err(e) => {
                    error!("{}", e);
                    unreachable!()
                }
            }
        })
    }

    fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        let delay_in_secs = delay.map_or(1, |duration| duration.as_secs());
        self.proto
            .send(
                "smartlife.iot.common.system",
                "reboot",
                Some(&json!({ "delay": delay_in_secs })),
            )
            .map(|res| match String::from_utf8(res) {
                Ok(res) => debug!("{}", res),
                Err(e) => error!("{}", e),
            })
    }

    fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        let delay_in_secs = delay.map_or(1, |duration| duration.as_secs());
        self.proto
            .send(
                "smartlife.iot.common.system",
                "reset",
                Some(&json!({ "delay": delay_in_secs })),
            )
            .map(|res| match String::from_utf8(res) {
                Ok(res) => debug!("{}", res),
                Err(e) => error!("{}", e),
            })
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

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    #[serde(rename = "system")]
    sys_info: LB110Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LB110Info {
    #[serde(rename = "get_sysinfo")]
    values: HashMap<String, Value>,
}

impl LB110Info {
    pub fn sw_ver(&self) -> Option<String> {
        self.get_string("sw_ver")
    }

    pub fn hw_ver(&self) -> Option<String> {
        self.get_string("hw_ver")
    }

    pub fn device_id(&self) -> Option<String> {
        self.get_string("deviceId")
    }

    pub fn alias(&self) -> Option<String> {
        self.get_string("alias")
    }

    pub fn model(&self) -> Option<String> {
        self.get_string("model")
    }

    pub fn device_type(&self) -> Option<String> {
        self.get_string("mic_type")
    }

    pub fn mac_address(&self) -> Option<String> {
        self.get_string("mic_mac")
    }

    fn get_string(&self, key: &str) -> Option<String> {
        self.values.get(key).map(|value| value.to_string())
    }
}

impl fmt::Display for LB110Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.values).unwrap())
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
