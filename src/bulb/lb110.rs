use crate::command::{Device, SysInfo, System};
use crate::error::{self, Result};
use crate::proto::{self, Proto};

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::fmt;
use std::net::IpAddr;
use std::time::Duration;

pub struct LB110 {
    proto: Proto,
}

impl LB110 {
    pub(super) fn new<A>(host: A) -> LB110
    where
        A: Into<IpAddr>,
    {
        LB110 {
            proto: proto::Builder::default(host),
        }
    }

    pub(super) fn sw_ver(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.sw_ver().to_string())
    }

    pub(super) fn hw_ver(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.hw_ver().to_string())
    }

    pub(super) fn model(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.model().to_string())
    }

    pub(super) fn alias(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.alias().to_string())
    }

    pub(super) fn mac_address(&self) -> Result<String> {
        self.sysinfo()
            .map(|sysinfo| sysinfo.mac_address().to_string())
    }

    pub(super) fn is_dimmable(&self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.is_dimmable())
    }

    pub(super) fn is_color(&self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.is_color())
    }

    pub(super) fn is_variable_color_temp(&self) -> Result<bool> {
        self.sysinfo()
            .map(|sysinfo| sysinfo.is_variable_color_temp())
    }

    pub(super) fn rssi(&self) -> Result<i64> {
        self.sysinfo().map(|sysinfo| sysinfo.rssi())
    }

    pub(super) fn hsv(&self) -> Result<HSV> {
        self.sysinfo().and_then(|sysinfo| sysinfo.hsv())
    }

    pub(super) fn is_on(&self) -> Result<bool> {
        self.get_light_state()
            .map(|light_state| light_state.is_on())
    }

    fn get_light_state(&self) -> Result<LightState> {
        self.proto
            .send(
                "smartlife.iot.smartbulb.lightingservice",
                "get_light_state",
                None,
            )
            .map(|res| {
                serde_json::from_slice::<Response>(&res)
                    .unwrap()
                    .lighting
                    .unwrap()
                    .light_state
            })
    }

    fn set_light_state(&self, arg: Option<&Value>) -> Result<LightState> {
        self.proto
            .send(
                "smartlife.iot.smartbulb.lightingservice",
                "transition_light_state",
                arg,
            )
            .map(|res| {
                serde_json::from_slice::<Response>(&res)
                    .unwrap()
                    .lighting
                    .unwrap()
                    .light_state
            })
    }
}

impl Device for LB110 {
    fn turn_on(&mut self) -> Result<()> {
        self.set_light_state(Some(&json!({ "on_off": 1 })))
            .map(|state| log::trace!("{:?}", state))
    }

    fn turn_off(&mut self) -> Result<()> {
        self.set_light_state(Some(&json!({ "on_off": 0 })))
            .map(|state| log::trace!("{:?}", state))
    }
}

impl System for LB110 {
    fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        let delay_in_secs = delay.map_or(1, |duration| duration.as_secs());
        self.proto
            .send(
                "smartlife.iot.common.system",
                "reboot",
                Some(&json!({ "delay": delay_in_secs })),
            )
            .map(|_| {})
    }

    fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        let delay_in_secs = delay.map_or(1, |duration| duration.as_secs());
        self.proto
            .send(
                "smartlife.iot.common.system",
                "reset",
                Some(&json!({ "delay": delay_in_secs })),
            )
            .map(|_| {})
    }
}

impl SysInfo for LB110 {
    type Info = LB110Info;

    fn sysinfo(&self) -> Result<Self::Info> {
        self.proto.send("system", "get_sysinfo", None).map(|res| {
            serde_json::from_slice::<Response>(&res)
                .unwrap()
                .system
                .unwrap()
                .get_sysinfo
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    #[serde(alias = "system")]
    system: Option<GetSysInfo>,

    #[serde(alias = "smartlife.iot.smartbulb.lightingservice")]
    lighting: Option<Lighting>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetSysInfo {
    get_sysinfo: LB110Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LB110Info {
    sw_ver: String,
    hw_ver: String,
    model: String,
    description: Option<String>,
    alias: String,
    mic_type: String,
    mic_mac: String,
    is_dimmable: u64,
    is_color: u64,
    is_variable_color_temp: u64,
    light_state: LightState,
    rssi: i64,
    #[serde(flatten)]
    other: Map<String, Value>,
}

impl LB110Info {
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
        &self.mic_mac
    }

    pub fn is_dimmable(&self) -> bool {
        self.is_dimmable == 1
    }

    pub fn is_color(&self) -> bool {
        self.is_color == 1
    }

    pub fn is_variable_color_temp(&self) -> bool {
        self.is_variable_color_temp == 1
    }

    pub fn rssi(&self) -> i64 {
        self.rssi
    }

    pub fn hsv(&self) -> Result<HSV> {
        if self.is_color == 1 {
            Ok(self.light_state.hsv())
        } else {
            Err(error::unsupported_operation("hsv"))
        }
    }
}

impl fmt::Display for LB110Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Lighting {
    #[serde(alias = "transition_light_state", alias = "get_light_state")]
    light_state: LightState,
}

#[derive(Debug, Serialize, Deserialize)]
struct LightState {
    on_off: u64,
    #[serde(flatten)]
    hsv: Option<HSV>,
    dft_on_state: Option<HSV>,
}

impl LightState {
    fn is_on(&self) -> bool {
        self.on_off == 1
    }

    fn hsv(&self) -> HSV {
        if self.on_off == 1 {
            self.hsv.as_ref().unwrap().clone()
        } else {
            self.dft_on_state.as_ref().unwrap().clone()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HSV {
    mode: Option<String>,
    hue: u64,
    saturation: u64,
    color_temp: u64,
    brightness: u64,
}

impl HSV {
    // degrees (0-360)
    pub fn hue(&self) -> u64 {
        self.hue
    }

    // % (0-100)
    pub fn saturation(&self) -> u64 {
        self.saturation
    }

    // % (0-100)
    pub fn value(&self) -> u64 {
        self.brightness
    }
}
