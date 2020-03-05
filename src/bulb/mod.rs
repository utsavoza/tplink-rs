use crate::command::{Device, SysInfo, System};
use crate::error::Result;
use crate::proto::{self, Proto};

use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::fmt;
use std::net::IpAddr;
use std::time::Duration;
use crate::{Error, ErrorKind};

pub struct Bulb<T> {
    device: T,
}

impl Bulb<LB1XX> {
    pub fn new<A>(host: A) -> Bulb<LB1XX>
    where
        A: Into<IpAddr>,
    {
        Bulb {
            device: LB1XX::new(host),
        }
    }

    pub fn sw_ver(&self) -> Result<String> {
        self.device.sw_ver()
    }

    pub fn hw_ver(&self) -> Result<String> {
        self.device.hw_ver()
    }

    pub fn model(&self) -> Result<String> {
        self.device.model()
    }

    pub fn alias(&self) -> Result<String> {
        self.device.alias()
    }

    pub fn mac_address(&self) -> Result<String> {
        self.device.mac_address()
    }

    pub fn is_dimmable(&self) -> Result<bool> {
        self.device.is_dimmable()
    }

    pub fn is_color(&self) -> Result<bool> {
        self.device.is_color()
    }

    pub fn is_variable_color_temp(&self) -> Result<bool> {
        self.device.is_variable_color_temp()
    }

    pub fn is_on(&self) -> Result<bool> {
        self.device.is_on()
    }

    pub fn hsv(&self) -> Result<(u64, u64, u64)> {
        self.device.hsv()
    }
}

impl<T: SysInfo> Bulb<T> {
    pub fn sysinfo(&self) -> Result<T::Info> {
        self.device.sysinfo()
    }
}

impl<T: System> Bulb<T> {
    pub fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.device.reboot(delay)
    }

    pub fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.device.factory_reset(delay)
    }
}

impl<T: Device> Bulb<T> {
    pub fn turn_on(&mut self) -> Result<()> {
        self.device.turn_on()
    }

    pub fn turn_off(&mut self) -> Result<()> {
        self.device.turn_off()
    }
}

pub struct LB1XX {
    proto: Proto,
}

impl LB1XX {
    fn new<A>(host: A) -> LB1XX
    where
        A: Into<IpAddr>,
    {
        LB1XX {
            proto: proto::Builder::default(host),
        }
    }

    fn sw_ver(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.sw_ver().to_string())
    }

    fn hw_ver(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.hw_ver().to_string())
    }

    fn model(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.model().to_string())
    }

    fn alias(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.alias().to_string())
    }

    fn mac_address(&self) -> Result<String> {
        self.sysinfo()
            .map(|sysinfo| sysinfo.mac_address().to_string())
    }

    fn is_dimmable(&self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.is_dimmable())
    }

    fn is_color(&self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.is_color())
    }

    fn is_variable_color_temp(&self) -> Result<bool> {
        self.sysinfo()
            .map(|sysinfo| sysinfo.is_variable_color_temp())
    }

    fn is_on(&self) -> Result<bool> {
        self.get_light_state()
            .map(|light_state| light_state.is_on())
    }

    fn hsv(&self) -> Result<(u64, u64, u64)> {
        self.sysinfo().and_then(|sysinfo| sysinfo.hsv())
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
    get_sysinfo: LB1XXInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LB1XXInfo {
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
    #[serde(flatten)]
    other: Map<String, Value>,
}

impl LB1XXInfo {
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

    pub fn hsv(&self) -> Result<(u64, u64, u64)> {
        if self.is_color == 1 {
            Ok(self.light_state.hsv())
        } else {
            Err(Error::new(ErrorKind::OperationNotSupported))
        }
    }
}

impl fmt::Display for LB1XXInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LightState {
    on_off: u64,
    #[serde(flatten)]
    hsv: Option<HSV>,
    dft_on_state: Option<HSV>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HSV {
    mode: Option<String>,
    hue: u64,
    saturation: u64,
    color_temp: u64,
    brightness: u64,
}

impl LightState {
    fn is_on(&self) -> bool {
        self.on_off == 1
    }

    fn hsv(&self) -> (u64, u64, u64) {
        if self.on_off == 1 {
            let &HSV {
                hue,
                saturation,
                brightness,
                ..
            } = self.hsv.as_ref().unwrap();
            (hue, saturation, brightness)
        } else {
            let &HSV {
                hue,
                saturation,
                brightness,
                ..
            } = self.dft_on_state.as_ref().unwrap();
            (hue, saturation, brightness)
        }
    }
}

impl SysInfo for LB1XX {
    type Info = LB1XXInfo;

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

impl System for LB1XX {
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

impl Device for LB1XX {
    fn turn_on(&mut self) -> Result<()> {
        self.set_light_state(Some(&json!({ "on_off": 1 })))
            .map(|state| debug!("{:?}", state))
    }

    fn turn_off(&mut self) -> Result<()> {
        self.set_light_state(Some(&json!({ "on_off": 0 })))
            .map(|state| debug!("{:?}", state))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Lighting {
    #[serde(alias = "transition_light_state", alias = "get_light_state")]
    light_state: LightState,
}

impl LB1XX {
    fn set_light_state(&self, arg: Option<&Value>) -> Result<LightState> {
        self.proto
            .send(
                "smartlife.iot.smartbulb.lightingservice",
                "transition_light_state",
                arg,
            )
            .map(|res| {
                println!("{}", std::str::from_utf8(&res).unwrap());
                serde_json::from_slice::<Response>(&res)
                    .unwrap()
                    .lighting
                    .unwrap()
                    .light_state
            })
    }

    fn get_light_state(&self) -> Result<LightState> {
        self.proto
            .send(
                "smartlife.iot.smartbulb.lightingservice",
                "get_light_state",
                None,
            )
            .map(|res| {
                println!("{}", std::str::from_utf8(&res).unwrap());
                serde_json::from_slice::<Response>(&res)
                    .unwrap()
                    .lighting
                    .unwrap()
                    .light_state
            })
    }
}
