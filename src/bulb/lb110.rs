use crate::bulb::lighting::{LightState, Lighting, HSV};
use crate::cache::Cache;
use crate::command::sys::System;
use crate::command::sysinfo::SystemInfo;
use crate::command::time::{DeviceTime, DeviceTimeZone, TimeSetting};
use crate::command::{Device, Sys, SysInfo, Time};
use crate::error::{self, Result};
use crate::proto::{self, Proto, Request};

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::cell::RefCell;
use std::fmt;
use std::net::IpAddr;
use std::time::Duration;

/// A TP-Link Wi-Fi LED Smart Bulb (LB110).
pub struct LB110 {
    proto: Proto,
    system: System,
    lighting: Lighting,
    time_setting: TimeSetting,
    sysinfo: SystemInfo<LB110Info>,
    cache: RefCell<Cache<Request, Value>>,
}

impl LB110 {
    pub(super) fn new<A>(host: A) -> LB110
    where
        A: Into<IpAddr>,
    {
        LB110 {
            proto: proto::Builder::default(host),
            system: System::new("smartlife.iot.common.system"),
            lighting: Lighting::new("smartlife.iot.smartbulb.lightingservice"),
            time_setting: TimeSetting::new("smartlife.iot.common.timesetting"),
            sysinfo: SystemInfo::new(),
            cache: RefCell::new(Cache::with_ttl(Duration::from_secs(3))),
        }
    }

    pub(super) fn sw_ver(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.sw_ver)
    }

    pub(super) fn hw_ver(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.hw_ver)
    }

    pub(super) fn model(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.model)
    }

    pub(super) fn alias(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.alias)
    }

    pub(super) fn mac_address(&self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.mic_mac)
    }

    pub(super) fn rssi(&self) -> Result<i64> {
        self.sysinfo().map(|sysinfo| sysinfo.rssi)
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

    pub(super) fn hsv(&self) -> Result<HSV> {
        self.sysinfo().and_then(|sysinfo| sysinfo.hsv())
    }

    pub(super) fn is_on(&self) -> Result<bool> {
        let mut cache = self.cache.borrow_mut();
        self.lighting
            .get_light_state(&self.proto, &mut cache)
            .map(|light_state| light_state.is_on())
    }

    pub(super) fn set_hue(&mut self, hue: u64) -> Result<()> {
        let is_color = self.sysinfo().map(|sysinfo| sysinfo.is_color())?;
        let is_valid_hue = (0..=360).contains(&hue);
        if is_color && is_valid_hue {
            let mut cache = self.cache.borrow_mut();
            self.lighting
                .set_light_state(&self.proto, Some(json!({ "hue": hue })), &mut cache)
                .map(|response| log::trace!("{:?}", response))
        } else if !is_valid_hue {
            Err(error::invalid_parameter(&format!(
                "set_hue: {} (valid range: 0-360)",
                hue
            )))
        } else {
            Err(error::unsupported_operation(&format!("set_hue: {}", hue)))
        }
    }

    pub(super) fn set_saturation(&mut self, saturation: u64) -> Result<()> {
        let is_color = self.sysinfo().map(|sysinfo| sysinfo.is_color())?;
        let is_valid_saturation = (0..=100).contains(&saturation);
        if is_color && is_valid_saturation {
            let mut cache = self.cache.borrow_mut();
            self.lighting
                .set_light_state(
                    &self.proto,
                    Some(json!({ "saturation": saturation })),
                    &mut cache,
                )
                .map(|response| log::trace!("{:?}", response))
        } else if !is_valid_saturation {
            Err(error::invalid_parameter(&format!(
                "set_saturation: {}% (valid range: 0-100%)",
                saturation
            )))
        } else {
            Err(error::unsupported_operation(&format!(
                "set_saturation: {}%",
                saturation
            )))
        }
    }

    pub(super) fn set_brightness(&mut self, brightness: u64) -> Result<()> {
        let is_dimmable = self.sysinfo().map(|sysinfo| sysinfo.is_dimmable())?;
        let is_valid_brightness = (0..=100).contains(&brightness);
        if is_dimmable && is_valid_brightness {
            let mut cache = self.cache.borrow_mut();
            self.lighting
                .set_light_state(
                    &self.proto,
                    Some(json!({ "brightness": brightness })),
                    &mut cache,
                )
                .map(|response| log::trace!("{:?}", response))
        } else if !is_valid_brightness {
            Err(error::invalid_parameter(&format!(
                "set_brightness: {}% (valid range: 0-100%)",
                brightness
            )))
        } else {
            Err(error::unsupported_operation(&format!(
                "set_brightness: {}%",
                brightness
            )))
        }
    }

    pub(super) fn brightness(&self) -> Result<u64> {
        let is_dimmable = self.sysinfo().map(|sysinfo| sysinfo.is_dimmable())?;
        if is_dimmable {
            let mut cache = self.cache.borrow_mut();
            let light_state = self.lighting.get_light_state(&self.proto, &mut cache)?;
            Ok(light_state.hsv().value())
        } else {
            Err(error::unsupported_operation("brightness"))
        }
    }
}

impl Device for LB110 {
    fn turn_on(&mut self) -> Result<()> {
        let mut cache = self.cache.borrow_mut();
        self.lighting
            .set_light_state(&self.proto, Some(json!({ "on_off": 1 })), &mut cache)
            .map(|state| log::trace!("{:?}", state))
    }

    fn turn_off(&mut self) -> Result<()> {
        let mut cache = self.cache.borrow_mut();
        self.lighting
            .set_light_state(&self.proto, Some(json!({ "on_off": 0 })), &mut cache)
            .map(|state| log::trace!("{:?}", state))
    }
}

impl Sys for LB110 {
    fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        let mut cache = self.cache.borrow_mut();
        self.system
            .reboot(&self.proto, delay, &mut cache)
            .map(|response| log::trace!("{:?}", response))
    }

    fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        let mut cache = self.cache.borrow_mut();
        self.system
            .factory_reset(&self.proto, delay, &mut cache)
            .map(|response| log::trace!("{:?}", response))
    }
}

impl Time for LB110 {
    fn time(&self) -> Result<DeviceTime> {
        self.time_setting.get_time(&self.proto)
    }

    fn timezone(&self) -> Result<DeviceTimeZone> {
        self.time_setting.get_timezone(&self.proto)
    }
}

impl SysInfo for LB110 {
    type Info = LB110Info;

    fn sysinfo(&self) -> Result<Self::Info> {
        let mut cache = self.cache.borrow_mut();
        self.sysinfo.get_sysinfo(&self.proto, &mut cache)
    }
}

/// The system information of TP-Link Smart Wi-Fi LED Bulb (LB110).
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
    /// Returns the software version of the device.
    pub fn sw_ver(&self) -> &str {
        &self.sw_ver
    }

    /// Returns the hardware version of the device.
    pub fn hw_ver(&self) -> &str {
        &self.hw_ver
    }

    /// Returns the model of the device.
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Returns the name (alias) of the device.
    pub fn alias(&self) -> &str {
        &self.alias
    }

    /// Returns the mac address of the device.
    pub fn mac_address(&self) -> &str {
        &self.mic_mac
    }

    /// Returns whether the bulb supports brightness changes.
    pub fn is_dimmable(&self) -> bool {
        self.is_dimmable == 1
    }

    /// Returns whether the bulb supports color changes.
    pub fn is_color(&self) -> bool {
        self.is_color == 1
    }

    /// Returns whether the bulb supports color temperature changes.
    pub fn is_variable_color_temp(&self) -> bool {
        self.is_variable_color_temp == 1
    }

    /// Returns the Wi-Fi signal strength (rssi) of the device.
    pub fn rssi(&self) -> i64 {
        self.rssi
    }

    /// Returns the current HSV (Hue, Saturation, Value) state of the bulb.
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
