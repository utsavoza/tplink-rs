use super::lighting::{LightState, Lighting, HSV};
use crate::cache::{Cache, ResponseCache};
use crate::cloud::{Cloud, CloudInfo, CloudSettings};
use crate::config::Config;
use crate::device::Device;
use crate::emeter::{DayStats, Emeter, EmeterStats, MonthStats, RealtimeStats};
use crate::error::{self, Result};
use crate::proto::{self, Proto};
use crate::sys::{Sys, System};
use crate::sysinfo::{SysInfo, SystemInfo};
use crate::time::{DeviceTime, DeviceTimeZone, Time, TimeSettings};
use crate::util;
use crate::wlan::{AccessPoint, Netif, Wlan};

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::cell::RefCell;
use std::fmt;
use std::net::IpAddr;
use std::rc::Rc;
use std::time::Duration;

/// A TP-Link Wi-Fi LED Smart Bulb (LB110).
pub struct LB110 {
    system: System,
    lighting: Lighting,
    time_settings: TimeSettings,
    cloud_settings: CloudSettings,
    netif: Netif,
    emeter: EmeterStats,
    sysinfo: SystemInfo<LB110Info>,
}

impl LB110 {
    pub(super) fn new<A>(host: A) -> LB110
    where
        A: Into<IpAddr>,
    {
        let proto = proto::Builder::default(host);
        let cache = Some(RefCell::new(Cache::with_ttl(Duration::from_secs(3))));
        LB110::with(proto, cache)
    }

    pub(super) fn with_config(config: Config) -> LB110 {
        let addr = config.addr;
        let read_timeout = config.read_timeout;
        let write_timeout = config.write_timeout;
        let buffer_size = config.buffer_size;

        let proto = proto::Builder::new(addr)
            .read_timeout(read_timeout)
            .write_timeout(write_timeout)
            .buffer_size(buffer_size)
            .build();

        let cache_config = config.cache_config;
        let cache = if cache_config.enable_cache {
            let ttl = cache_config.ttl.unwrap_or(Duration::from_secs(3));
            let cache = cache_config.initial_capacity.map_or_else(
                || Cache::with_ttl(ttl),
                |capacity| Cache::with_ttl_and_capacity(ttl, capacity),
            );
            Some(RefCell::new(cache))
        } else {
            None
        };

        LB110::with(proto, cache)
    }

    fn with(proto: Proto, cache: ResponseCache) -> LB110 {
        let proto = Rc::new(proto);
        let cache = Rc::new(cache);

        LB110 {
            system: System::new("smartlife.iot.common.system", proto.clone(), cache.clone()),
            lighting: Lighting::new(
                "smartlife.iot.smartbulb.lightingservice",
                proto.clone(),
                cache.clone(),
            ),
            cloud_settings: CloudSettings::new(
                "smartlife.iot.common.cloud",
                proto.clone(),
                cache.clone(),
            ),
            emeter: EmeterStats::new("smartlife.iot.common.emeter", proto.clone(), cache.clone()),
            time_settings: TimeSettings::new("smartlife.iot.common.timesetting", proto.clone()),
            netif: Netif::new(proto.clone()),
            sysinfo: SystemInfo::new(proto, cache),
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

    pub(super) fn is_on(&self) -> Result<bool> {
        self.lighting
            .get_light_state()
            .map(|light_state| light_state.is_on())
    }

    pub(super) fn has_emeter(&mut self) -> Result<bool> {
        Ok(true)
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
                self.lighting.set_light_state(Some(json!({
                    "hue": hue,
                    "saturation": saturation,
                    "value": value,
                    "color_temp": 0,
                })))
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
                    .set_light_state(Some(json!({ "hue": hue, "color_temp": 0 })))
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
                .get_light_state()
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
                    .set_light_state(Some(json!({ "saturation": saturation, "color_temp": 0 })))
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
                .get_light_state()
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
                    .set_light_state(Some(json!({ "brightness": brightness })))
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
                .get_light_state()
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
                    .set_light_state(Some(json!({ "color_temp": color_temp })))
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
                .get_light_state()
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
        self.lighting.set_light_state(Some(json!({ "on_off": 1 })))
    }

    fn turn_off(&mut self) -> Result<()> {
        self.lighting.set_light_state(Some(json!({ "on_off": 0 })))
    }
}

impl Sys for LB110 {
    fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.system.reboot(delay)
    }

    fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.system.reset(delay)
    }
}

impl Time for LB110 {
    fn time(&mut self) -> Result<DeviceTime> {
        self.time_settings.get_time()
    }

    fn timezone(&mut self) -> Result<DeviceTimeZone> {
        self.time_settings.get_timezone()
    }
}

impl Cloud for LB110 {
    fn get_cloud_info(&mut self) -> Result<CloudInfo> {
        self.cloud_settings.get_info()
    }

    fn bind(&mut self, username: &str, password: &str) -> Result<()> {
        self.cloud_settings.bind(username, password)
    }

    fn unbind(&mut self) -> Result<()> {
        self.cloud_settings.unbind()
    }

    fn get_firmware_list(&mut self) -> Result<Vec<String>> {
        self.cloud_settings.get_firmware_list()
    }

    fn set_server_url(&mut self, url: &str) -> Result<()> {
        self.cloud_settings.set_server_url(url)
    }
}

impl Wlan for LB110 {
    fn get_scan_info(
        &mut self,
        refresh: bool,
        timeout: Option<Duration>,
    ) -> Result<Vec<AccessPoint>> {
        self.netif.get_scan_info(refresh, timeout)
    }
}

impl Emeter for LB110 {
    fn get_emeter_realtime(&mut self) -> Result<RealtimeStats> {
        let (has_emeter, model) = self
            .sysinfo()
            .map(|sysinfo| (sysinfo.has_emeter(), sysinfo.model))?;

        if has_emeter {
            self.emeter.get_realtime()
        } else {
            Err(error::unsupported_operation(&format!(
                "{} get_emeter_realtime",
                model
            )))
        }
    }

    fn get_emeter_month_stats(&mut self, year: u32) -> Result<MonthStats> {
        let (has_emeter, model) = self
            .sysinfo()
            .map(|sysinfo| (sysinfo.has_emeter(), sysinfo.model))?;

        if has_emeter {
            self.emeter.get_month_stats(year)
        } else {
            Err(error::unsupported_operation(&format!(
                "{} get_emeter_month_stats",
                model
            )))
        }
    }

    fn get_emeter_day_stats(&mut self, month: u32, year: u32) -> Result<DayStats> {
        let (has_emeter, model) = self
            .sysinfo()
            .map(|sysinfo| (sysinfo.has_emeter(), sysinfo.model))?;

        if has_emeter {
            if util::u32_in_range(month, 1, 12) {
                self.emeter.get_day_stats(month, year)
            } else {
                Err(error::invalid_parameter(&format!(
                    "{} get_emeter_day_stats: month={} (valid range: 1-12)",
                    model, month
                )))
            }
        } else {
            Err(error::unsupported_operation(&format!(
                "{} get_emeter_day_stats",
                model
            )))
        }
    }

    fn erase_emeter_stats(&mut self) -> Result<()> {
        let (has_emeter, model) = self
            .sysinfo()
            .map(|sysinfo| (sysinfo.has_emeter(), sysinfo.model))?;

        if has_emeter {
            self.emeter.erase_stats()
        } else {
            Err(error::unsupported_operation(&format!(
                "{} erase_emeter_stats",
                model
            )))
        }
    }
}

impl SysInfo for LB110 {
    type Info = LB110Info;

    fn sysinfo(&mut self) -> Result<Self::Info> {
        self.sysinfo.get_sysinfo()
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

    pub fn has_emeter(&self) -> bool {
        true
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
