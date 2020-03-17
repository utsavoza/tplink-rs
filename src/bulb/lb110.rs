use super::lighting::{LightState, Lighting, HSV};
use crate::cache::Cache;
use crate::cloud::{Cloud, CloudInfo, CloudSettings};
use crate::device::Device;
use crate::error::{self, Result};
use crate::proto::{self, Proto, Request};
use crate::sys::{Sys, System};
use crate::sysinfo::{SysInfo, SystemInfo};
use crate::time::{DeviceTime, DeviceTimeZone, Time, TimeSettings};
use crate::util;
use crate::wlan::{AccessPoint, Netif, Wlan};

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::fmt;
use std::net::IpAddr;
use std::time::Duration;

/// A TP-Link Wi-Fi LED Smart Bulb (LB110).
pub struct LB110 {
    proto: Proto,
    system: System,
    lighting: Lighting,
    time_setting: TimeSettings,
    cloud_setting: CloudSettings,
    netif: Netif,
    sysinfo: SystemInfo<LB110Info>,
    cache: Option<Cache<Request, Value>>,
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
            time_setting: TimeSettings::new("smartlife.iot.common.timesetting"),
            cloud_setting: CloudSettings::new("smartlife.iot.common.cloud"),
            netif: Netif::new(),
            sysinfo: SystemInfo::new(),
            cache: Some(Cache::with_ttl(Duration::from_secs(3))),
        }
    }

    pub(super) fn sw_ver(&mut self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.sw_ver)
    }

    pub(super) fn hw_ver(&mut self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.hw_ver)
    }

    pub(super) fn model(&mut self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.model)
    }

    pub(super) fn alias(&mut self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.alias)
    }

    pub(super) fn mac_address(&mut self) -> Result<String> {
        self.sysinfo().map(|sysinfo| sysinfo.mic_mac)
    }

    pub(super) fn rssi(&mut self) -> Result<i64> {
        self.sysinfo().map(|sysinfo| sysinfo.rssi)
    }

    pub(super) fn is_dimmable(&mut self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.is_dimmable())
    }

    pub(super) fn is_color(&mut self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.is_color())
    }

    pub(super) fn is_variable_color_temp(&mut self) -> Result<bool> {
        self.sysinfo()
            .map(|sysinfo| sysinfo.is_variable_color_temp())
    }

    pub(super) fn is_on(&mut self) -> Result<bool> {
        self.lighting
            .get_light_state(&self.proto, self.cache.as_mut())
            .map(|light_state| light_state.is_on())
    }

    pub(super) fn hsv(&mut self) -> Result<HSV> {
        self.sysinfo().and_then(|sysinfo| sysinfo.hsv())
    }

    pub(super) fn set_hsv(&mut self, hue: u32, saturation: u32, value: u32) -> Result<()> {
        let (is_color, model) = self
            .sysinfo()
            .map(|sysinfo| (sysinfo.is_color(), sysinfo.model))?;
        if is_color {
            if util::u32_in_range(hue, 0, 360)
                && util::u32_in_range(saturation, 0, 100)
                && util::u32_in_range(value, 0, 100)
            {
                self.lighting
                    .set_light_state(
                        &self.proto,
                        self.cache.as_mut(),
                        Some(json!({
                            "hue": hue,
                            "saturation": saturation,
                            "value": value,
                            "color_temp": 0,
                        })),
                    )
                    .map(|_| {})
            } else {
                Err(error::invalid_parameter(&format!(
                    "{} set_hsv: ({}°, {}%, {}%) (valid range: hue(0-360°), saturation(0-100%), value(0-100%))",
                    model, hue, saturation, value
                )))
            }
        } else {
            Err(error::unsupported_operation(&format!(
                "{} set_hsv: ({}°, {}%, {}%)",
                model, hue, saturation, value
            )))
        }
    }

    pub(super) fn set_hue(&mut self, hue: u32) -> Result<()> {
        let (is_color, model) = self
            .sysinfo()
            .map(|sysinfo| (sysinfo.is_color(), sysinfo.model))?;
        if is_color {
            if util::u32_in_range(hue, 0, 360) {
                self.lighting
                    .set_light_state(
                        &self.proto,
                        self.cache.as_mut(),
                        Some(json!({ "hue": hue, "color_temp": 0 })),
                    )
                    .map(|_| {})
            } else {
                Err(error::invalid_parameter(&format!(
                    "{} set_hue: {}° (valid range: 0-360°)",
                    model, hue
                )))
            }
        } else {
            Err(error::unsupported_operation(&format!(
                "{} set_hue: {}°",
                model, hue
            )))
        }
    }

    pub(super) fn hue(&mut self) -> Result<u32> {
        let (is_color, model) = self
            .sysinfo()
            .map(|sysinfo| (sysinfo.is_color(), sysinfo.model))?;
        if is_color {
            self.lighting
                .get_light_state(&self.proto, self.cache.as_mut())
                .map(|light_state| light_state.hsv().hue())
        } else {
            Err(error::unsupported_operation(&format!("{} hue", model)))
        }
    }

    pub(super) fn set_saturation(&mut self, saturation: u32) -> Result<()> {
        let (is_color, model) = self
            .sysinfo()
            .map(|sysinfo| (sysinfo.is_color(), sysinfo.model))?;
        if is_color {
            if util::u32_in_range(saturation, 0, 100) {
                self.lighting
                    .set_light_state(
                        &self.proto,
                        self.cache.as_mut(),
                        Some(json!({ "saturation": saturation, "color_temp": 0 })),
                    )
                    .map(|_| {})
            } else {
                Err(error::invalid_parameter(&format!(
                    "{} set_saturation: {}% (valid range: 0-100%)",
                    model, saturation
                )))
            }
        } else {
            Err(error::unsupported_operation(&format!(
                "{} set_saturation: {}%",
                model, saturation
            )))
        }
    }

    pub(super) fn saturation(&mut self) -> Result<u32> {
        let (is_color, model) = self
            .sysinfo()
            .map(|sysinfo| (sysinfo.is_color(), sysinfo.model))?;
        if is_color {
            self.lighting
                .get_light_state(&self.proto, self.cache.as_mut())
                .map(|light_state| light_state.hsv().saturation())
        } else {
            Err(error::unsupported_operation(&format!(
                "{} saturation",
                model
            )))
        }
    }

    pub(super) fn set_brightness(&mut self, brightness: u32) -> Result<()> {
        let (is_dimmable, model) = self
            .sysinfo()
            .map(|sysinfo| (sysinfo.is_dimmable(), sysinfo.model))?;
        if is_dimmable {
            if util::u32_in_range(brightness, 0, 100) {
                self.lighting
                    .set_light_state(
                        &self.proto,
                        self.cache.as_mut(),
                        Some(json!({ "brightness": brightness })),
                    )
                    .map(|_| {})
            } else {
                Err(error::invalid_parameter(&format!(
                    "{} set_brightness: {}% (valid range: 0-100%)",
                    model, brightness
                )))
            }
        } else {
            Err(error::unsupported_operation(&format!(
                "{} set_brightness: {}%",
                model, brightness
            )))
        }
    }

    pub(super) fn brightness(&mut self) -> Result<u32> {
        let (is_dimmable, model) = self
            .sysinfo()
            .map(|sysinfo| (sysinfo.is_dimmable(), sysinfo.model))?;
        if is_dimmable {
            self.lighting
                .get_light_state(&self.proto, self.cache.as_mut())
                .map(|light_state| light_state.hsv().value())
        } else {
            Err(error::unsupported_operation(&format!(
                "{} brightness",
                model
            )))
        }
    }

    pub(super) fn set_color_temp(&mut self, color_temp: u32) -> Result<()> {
        let (is_variable_color_temp, model) = self
            .sysinfo()
            .map(|sysinfo| (sysinfo.is_variable_color_temp(), sysinfo.model))?;
        if is_variable_color_temp {
            let range = util::valid_color_temp_range(&model);
            if util::u32_in_range(color_temp, range.0, range.1) {
                self.lighting
                    .set_light_state(
                        &self.proto,
                        self.cache.as_mut(),
                        Some(json!({ "color_temp": color_temp })),
                    )
                    .map(|_| {})
            } else {
                Err(error::invalid_parameter(&format!(
                    "{} set_color_temp: {} (valid range: {}-{}K)",
                    model, color_temp, range.0, range.1
                )))
            }
        } else {
            Err(error::unsupported_operation(&format!(
                "{} set_color_temp: {}K",
                model, color_temp
            )))
        }
    }

    pub(super) fn color_temp(&mut self) -> Result<u32> {
        let (is_variable_color_temp, model) = self
            .sysinfo()
            .map(|sysinfo| (sysinfo.is_variable_color_temp(), sysinfo.model))?;
        if is_variable_color_temp {
            self.lighting
                .get_light_state(&self.proto, self.cache.as_mut())
                .map(|light_state| light_state.hsv().color_temp())
        } else {
            Err(error::unsupported_operation(&format!(
                "{} color_temp",
                model
            )))
        }
    }
}

impl Device for LB110 {
    fn turn_on(&mut self) -> Result<()> {
        self.lighting.set_light_state(
            &self.proto,
            self.cache.as_mut(),
            Some(json!({ "on_off": 1 })),
        )
    }

    fn turn_off(&mut self) -> Result<()> {
        self.lighting.set_light_state(
            &self.proto,
            self.cache.as_mut(),
            Some(json!({ "on_off": 0 })),
        )
    }
}

impl Sys for LB110 {
    fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.system.reboot(&self.proto, self.cache.as_mut(), delay)
    }

    fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.system.reset(&self.proto, self.cache.as_mut(), delay)
    }
}

impl Time for LB110 {
    fn time(&mut self) -> Result<DeviceTime> {
        self.time_setting.get_time(&self.proto)
    }

    fn timezone(&mut self) -> Result<DeviceTimeZone> {
        self.time_setting.get_timezone(&self.proto)
    }
}

impl Cloud for LB110 {
    fn get_cloud_info(&mut self) -> Result<CloudInfo> {
        self.cloud_setting
            .get_info(&self.proto, self.cache.as_mut())
    }

    fn bind(&mut self, username: &str, password: &str) -> Result<()> {
        self.cloud_setting
            .bind(&self.proto, self.cache.as_mut(), username, password)
    }

    fn unbind(&mut self) -> Result<()> {
        self.cloud_setting.unbind(&self.proto, self.cache.as_mut())
    }

    fn get_firmware_list(&mut self) -> Result<Vec<String>> {
        self.cloud_setting
            .get_firmware_list(&self.proto, self.cache.as_mut())
    }

    fn set_server_url(&mut self, url: &str) -> Result<()> {
        self.cloud_setting
            .set_server_url(&self.proto, self.cache.as_mut(), url)
    }
}

impl Wlan for LB110 {
    fn get_scan_info(
        &mut self,
        refresh: bool,
        timeout: Option<Duration>,
    ) -> Result<Vec<AccessPoint>> {
        self.netif.get_scan_info(&self.proto, refresh, timeout)
    }
}

impl SysInfo for LB110 {
    type Info = LB110Info;

    fn sysinfo(&mut self) -> Result<Self::Info> {
        self.sysinfo.get_sysinfo(&self.proto, self.cache.as_mut())
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
