use crate::error::Result;
use crate::proto::{self, Proto};
use crate::system::System;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt;
use std::net::IpAddr;

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

impl<T> Plug<T>
where
    T: System,
{
    pub fn sys_info(&self) -> Result<T::Info> {
        self.model.sys_info()
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
        HS100 {
            proto: proto::Builder::new(host).build(),
        }
    }
}

impl System for HS100 {
    type Info = SystemInfo;

    fn sys_info(&self) -> Result<Self::Info> {
        self.proto
            .send_command(&json!({"system":{"get_sysinfo":{}}}))
            .map(|res| {
                serde_json::from_slice::<Response>(&res)
                    .expect("invalid system response")
                    .sys_info
            })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    #[serde(rename = "system")]
    sys_info: SystemInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    #[serde(rename = "get_sysinfo")]
    values: HashMap<String, Value>,
}

impl SystemInfo {
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

impl fmt::Display for SystemInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.values).unwrap())
    }
}
