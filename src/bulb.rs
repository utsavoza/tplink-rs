use crate::device::Device;
use crate::error::Result;
use crate::proto::{self, Proto};
use crate::system::System;

use log::{debug, error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt;
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

impl<T: System> Bulb<T> {
    pub fn sys_info(&mut self) -> Result<T::SystemInfo> {
        self.model.sys_info()
    }

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
}

impl LB110 {
    fn new<A>(host: A) -> LB110
    where
        A: Into<IpAddr>,
    {
        LB110 {
            proto: proto::Builder::default(host),
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
        self.proto
            .send(
                "smartlife.iot.smartbulb.lightingservice",
                "transition_light_state",
                Some(&json!({"on_off": 1})),
            )
            .map(|res| match String::from_utf8(res) {
                Ok(res) => debug!("{}", res),
                Err(e) => error!("{}", e),
            })
    }

    fn turn_off(&mut self) -> Result<()> {
        self.proto
            .send(
                "smartlife.iot.smartbulb.lightingservice",
                "transition_light_state",
                Some(&json!({"on_off": 0})),
            )
            .map(|res| match String::from_utf8(res) {
                Ok(res) => debug!("{}", res),
                Err(e) => error!("{}", e),
            })
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
