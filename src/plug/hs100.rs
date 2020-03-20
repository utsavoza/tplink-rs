use super::timer::{Rule, RuleList, Timer, TimerSettings};
use crate::cache::{Cache, ResponseCache};
use crate::cloud::{Cloud, CloudInfo, CloudSettings};
use crate::device::Device;
use crate::emeter::{DayStats, Emeter, EmeterStats, MonthStats, RealtimeStats};
use crate::error::{self, Result};
use crate::proto::{self, Proto, Request};
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

/// A TP-Link Wi-Fi Smart Plug (HS100).
pub struct HS100 {
    proto: Rc<Proto>,
    cache: Rc<ResponseCache>,
    system: System,
    time_settings: TimeSettings,
    timer_settings: TimerSettings,
    cloud_settings: CloudSettings,
    emeter: EmeterStats,
    netif: Netif,
    sysinfo: SystemInfo<HS100Info>,
}

impl HS100 {
    pub(super) fn new<A>(host: A) -> HS100
    where
        A: Into<IpAddr>,
    {
        let proto = Rc::new(proto::Builder::default(host));
        let cache = Rc::new(Some(RefCell::new(Cache::with_ttl(Duration::from_secs(3)))));
        let system = System::new("system", Rc::clone(&proto), Rc::clone(&cache));
        let time_settings = TimeSettings::new("time", Rc::clone(&proto));
        let timer_settings = TimerSettings::new("count_down", Rc::clone(&proto), Rc::clone(&cache));
        let cloud_settings = CloudSettings::new("cnCloud", Rc::clone(&proto), Rc::clone(&cache));
        let emeter = EmeterStats::new("emeter", Rc::clone(&proto), Rc::clone(&cache));
        let netif = Netif::new(Rc::clone(&proto));
        let sysinfo = SystemInfo::new(Rc::clone(&proto), Rc::clone(&cache));

        HS100 {
            system,
            time_settings,
            timer_settings,
            cloud_settings,
            emeter,
            netif,
            sysinfo,
            proto,
            cache,
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
        self.sysinfo().map(|sysinfo| sysinfo.mac)
    }

    pub(super) fn rssi(&mut self) -> Result<i64> {
        self.sysinfo().map(|sysinfo| sysinfo.rssi)
    }

    pub(super) fn location(&mut self) -> Result<Location> {
        self.sysinfo().map(|sysinfo| sysinfo.location)
    }

    pub(super) fn has_emeter(&mut self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.has_emeter())
    }

    pub(super) fn is_on(&mut self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.is_on())
    }

    pub(super) fn is_led_on(&mut self) -> Result<bool> {
        self.sysinfo().map(|sysinfo| sysinfo.is_led_on())
    }

    pub(super) fn turn_on_led(&mut self) -> Result<()> {
        if let Some(cache) = self.cache.as_ref() {
            cache.borrow_mut().retain(|k, _| k.target != "system");
        }

        let response = self.proto.send_request(&Request::new(
            "system",
            "set_led_off",
            Some(json!({ "off": false })),
        ))?;

        log::trace!("(system) {:?}", response);

        Ok(())
    }

    pub(super) fn turn_off_led(&mut self) -> Result<()> {
        if let Some(cache) = self.cache.as_ref() {
            cache.borrow_mut().retain(|k, _| k.target != "system");
        }

        let response = self.proto.send_request(&Request::new(
            "system",
            "set_led_off",
            Some(json!({ "off": true })),
        ))?;

        log::trace!("(system) {:?}", response);

        Ok(())
    }
}

impl Device for HS100 {
    fn turn_on(&mut self) -> Result<()> {
        if let Some(cache) = self.cache.as_ref() {
            cache.borrow_mut().retain(|k, _| k.target != "system");
        }

        let response = self.proto.send_request(&Request::new(
            "system",
            "set_relay_state",
            Some(json!({ "state": 1 })),
        ))?;

        log::trace!("(system) {:?}", response);

        Ok(())
    }

    fn turn_off(&mut self) -> Result<()> {
        if let Some(cache) = self.cache.as_ref() {
            cache.borrow_mut().retain(|k, _| k.target != "system");
        }

        let response = self.proto.send_request(&Request::new(
            "system",
            "set_relay_state",
            Some(json!({ "state": 0 })),
        ))?;

        log::trace!("(system) {:?}", response);

        Ok(())
    }
}

impl Sys for HS100 {
    fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.system.reboot(delay)
    }

    fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.system.reset(delay)
    }
}

impl Time for HS100 {
    fn time(&mut self) -> Result<DeviceTime> {
        self.time_settings.get_time()
    }

    fn timezone(&mut self) -> Result<DeviceTimeZone> {
        self.time_settings.get_timezone()
    }
}

impl Timer for HS100 {
    fn get_timer_rules(&mut self) -> Result<RuleList> {
        self.timer_settings.get_rules()
    }

    fn add_timer_rule(&mut self, rule: Rule) -> Result<String> {
        let is_table_empty = self.get_timer_rules().map(|list| list.is_empty())?;
        if is_table_empty {
            self.timer_settings.add_rule(rule)
        } else {
            Err(error::unsupported_operation(
                "add_timer_rule: table is full",
            ))
        }
    }

    fn edit_timer_rule(&mut self, id: &str, rule: Rule) -> Result<()> {
        self.timer_settings.edit_rule(id, rule)
    }

    fn delete_timer_rule_with_id(&mut self, id: &str) -> Result<()> {
        self.timer_settings.delete_rule_with_id(id)
    }

    fn delete_all_timer_rules(&mut self) -> Result<()> {
        self.timer_settings.delete_all_rules()
    }
}

impl Cloud for HS100 {
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

impl Wlan for HS100 {
    fn get_scan_info(
        &mut self,
        refresh: bool,
        timeout: Option<Duration>,
    ) -> Result<Vec<AccessPoint>> {
        self.netif.get_scan_info(refresh, timeout)
    }
}

impl Emeter for HS100 {
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

impl SysInfo for HS100 {
    type Info = HS100Info;

    fn sysinfo(&mut self) -> Result<Self::Info> {
        self.sysinfo.get_sysinfo()
    }
}

/// The system information of TP-Link Wi-Fi Smart Plug (HS100).
#[derive(Debug, Serialize, Deserialize)]
pub struct HS100Info {
    sw_ver: String,
    hw_ver: String,
    model: String,
    #[serde(rename = "type")]
    device_type: String,
    mac: String,
    alias: String,
    relay_state: u64,
    rssi: i64,
    #[serde(flatten)]
    location: Location,
    led_off: u64,
    feature: String,
    #[serde(flatten)]
    other: Map<String, Value>,
}

/// The location coordinates of the device.
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "longitude_i")]
    pub longitude: i64,
    #[serde(rename = "latitude_i")]
    pub latitude: i64,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.latitude, self.longitude)
    }
}

impl HS100Info {
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
        &self.mac
    }

    /// Returns the Wi-Fi signal strength (rssi) of the device.
    pub fn rssi(&self) -> i64 {
        self.rssi
    }

    /// Returns the location of the device.
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// Returns whether the device supports emeter stats.
    pub fn has_emeter(&self) -> bool {
        self.feature.contains("ENE")
    }

    /// Returns whether the device is on.
    fn is_on(&self) -> bool {
        self.relay_state == 1
    }

    /// Returns whether the device LED is on.
    fn is_led_on(&self) -> bool {
        self.led_off == 0
    }
}

impl fmt::Display for HS100Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
