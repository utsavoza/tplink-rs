use crate::device::Device;
use crate::error::{self, Result};
use crate::proto::{self, Proto};
use crate::system::System;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::net::IpAddr;
use std::time::Duration;

pub struct Plug<T> {
    model: T,
}

impl Plug<HS100> {
    pub fn new<A>(host: A) -> Plug<HS100>
    where
        A: Into<IpAddr>,
    {
        Plug {
            model: HS100::new(host),
        }
    }
}

impl<T: System> Plug<T> {
    pub fn sys_info(&mut self) -> Result<T::SystemInfo> {
        self.model.sys_info()
    }
}

impl<T: Device> Plug<T> {
    pub fn turn_on(&self) -> Result<()> {
        self.model.turn_on()
    }

    pub fn turn_off(&self) -> Result<()> {
        self.model.turn_off()
    }
}

pub struct HS100 {
    proto: Proto,
}

impl HS100 {
    fn new<A>(host: A) -> HS100
    where
        A: Into<IpAddr>,
    {
        let proto = proto::Builder::new(host)
            .read_timeout(Duration::from_secs(3))
            .write_timeout(Duration::from_secs(3))
            .cache_config(Duration::from_secs(3), None)
            .build();

        HS100 { proto }
    }
}

impl System for HS100 {
    type SystemInfo = HS100Info;

    fn sys_info(&mut self) -> Result<Self::SystemInfo> {
        self.proto.send("system", "get_sysinfo", None).map(|res| {
            serde_json::from_slice::<Response>(&res)
                .map(|res| res.sys_info)
                .map_err(error::json)
        })?
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    #[serde(rename = "system")]
    sys_info: HS100Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HS100Info {
    #[serde(rename = "get_sysinfo")]
    values: HashMap<String, Value>,
}

impl HS100Info {
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
        self.get_string("type")
    }

    pub fn mac_address(&self) -> Option<String> {
        self.get_string("mac")
    }

    fn get_string(&self, key: &str) -> Option<String> {
        self.values.get(key).map(|value| value.to_string())
    }
}

impl fmt::Display for HS100Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.values).unwrap())
    }
}
