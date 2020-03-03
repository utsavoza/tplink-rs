use crate::error::Result;
use crate::proto::Proto;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    #[serde(rename = "system")]
    info: LB110Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LB110Info {
    #[serde(rename = "get_sysinfo")]
    values: Map<String, Value>,
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

    pub fn is_dimmable(&self) -> Option<bool> {
        self.get_i64("is_dimmable").map(|value| value == 1)
    }

    pub fn is_color(&self) -> Option<bool> {
        self.get_i64("is_color").map(|value| value == 1)
    }

    pub fn is_variable_color_temp(&self) -> Option<bool> {
        self.get_i64("is_variable_color_temp")
            .map(|value| value == 1)
    }

    fn get_string(&self, key: &str) -> Option<String> {
        self.values.get(key).map(|value| value.to_string())
    }

    fn get_i64(&self, key: &str) -> Option<i64> {
        self.values.get(key).and_then(|value| value.as_i64())
    }
}

pub(super) struct Info(String);

impl Info {
    pub(super) fn new(namespace: Option<&str>) -> Info {
        Info(namespace.unwrap_or("system").into())
    }

    pub(super) fn get_sysinfo(&self, proto: &mut Proto) -> Result<LB110Info> {
        proto
            .send(&self.0, "get_sysinfo", None)
            .map(|res| serde_json::from_slice::<Response>(&res).unwrap().info)
    }
}
