mod lb110;
mod lighting;

pub use self::lb110::LB110;
use crate::bulb::lighting::HSV;
use crate::cloud::{Cloud, CloudInfo};
use crate::device::Device;
use crate::emeter::{DayStats, Emeter, MonthStats, RealtimeStats};
use crate::error::Result;
use crate::sys::Sys;
use crate::sysinfo::SysInfo;
use crate::time::{DeviceTime, DeviceTimeZone, Time};
use crate::wlan::{AccessPoint, Wlan};

use std::net::IpAddr;
use std::time::Duration;

/// A TP-Link Smart Bulb.
///
/// # Examples
///
/// ```no_run
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
///
///     bulb.turn_on()?;
///     assert_eq!(bulb.is_on()?, true);
///
///     bulb.turn_off()?;
///     assert_eq!(bulb.is_on()?, false);
///
///     Ok(())
/// }
/// ```
pub struct Bulb<T> {
    device: T,
}

impl<T: Device> Bulb<T> {
    /// Turns on the bulb.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// bulb.turn_on()?;
    /// assert_eq!(bulb.is_on()?, true);
    /// # Ok(())
    /// # }
    /// ```
    pub fn turn_on(&mut self) -> Result<()> {
        self.device.turn_on()
    }

    /// Turns off the bulb.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// bulb.turn_off()?;
    /// assert_eq!(bulb.is_on()?, false);
    /// # Ok(())
    /// # }
    /// ```
    pub fn turn_off(&mut self) -> Result<()> {
        self.device.turn_off()
    }
}

impl<T: Sys> Bulb<T> {
    /// Reboots the bulb after the given duration. In case when
    /// the delay duration is not provided, the bulb is set to
    /// reboot after a default delay of 1 second.
    ///
    /// # Examples
    /// Reboots the bulb after a delay of 3 seconds.
    ///
    /// ```no_run
    /// use std::time::Duration;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// bulb.reboot(Some(Duration::from_secs(3)))?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Reboots the bulb after 1 second.
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// bulb.reboot(None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.device.reboot(delay)
    }

    /// Factory resets the bulb after the given duration. In case when the delay
    /// duration is not provided, the bulb is set to reset after a default delay
    /// of 1 second.
    ///
    /// # Examples
    /// Factory resets the bulb after a delay for 3 seconds.
    ///
    /// ```no_run
    /// use std::time::Duration;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// bulb.factory_reset(Some(Duration::from_secs(3)))?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Factory resets the bulb after 1 second.
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// bulb.factory_reset(None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.device.factory_reset(delay)
    }
}

impl<T: Time> Bulb<T> {
    /// Returns the current date and time of the device without the timezone.
    /// To get the device timezone, use [`timezone`] method.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let device_time = bulb.time()?;
    /// println!("{}", device_time); // e.g. `2020-04-08 22:29:07`
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`timezone`]: #method.timezone
    pub fn time(&mut self) -> Result<DeviceTime> {
        self.device.time()
    }

    /// Returns the current timezone of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// bulb.timezone()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn timezone(&mut self) -> Result<DeviceTimeZone> {
        self.device.timezone()
    }
}

impl<T: Cloud> Bulb<T> {
    pub fn get_cloud_info(&mut self) -> Result<CloudInfo> {
        self.device.get_cloud_info()
    }

    pub fn bind(&mut self, username: &str, password: &str) -> Result<()> {
        self.device.bind(username, password)
    }

    pub fn unbind(&mut self) -> Result<()> {
        self.device.unbind()
    }

    pub fn get_firmware_list(&mut self) -> Result<Vec<String>> {
        self.device.get_firmware_list()
    }

    pub fn set_server_url(&mut self, url: &str) -> Result<()> {
        self.device.set_server_url(url)
    }
}

impl<T: Wlan> Bulb<T> {
    pub fn get_scan_info(
        &mut self,
        refresh: bool,
        timeout: Option<Duration>,
    ) -> Result<Vec<AccessPoint>> {
        self.device.get_scan_info(refresh, timeout)
    }
}

impl<T: Emeter> Bulb<T> {
    pub fn get_emeter_realtime(&mut self) -> Result<RealtimeStats> {
        self.device.get_emeter_realtime()
    }

    pub fn get_emeter_month_stats(&mut self, year: u32) -> Result<MonthStats> {
        self.device.get_emeter_month_stats(year)
    }

    pub fn get_emeter_day_stats(&mut self, month: u32, year: u32) -> Result<DayStats> {
        self.device.get_emeter_day_stats(month, year)
    }

    pub fn erase_emeter_stats(&mut self) -> Result<()> {
        self.device.erase_emeter_stats()
    }
}

impl<T: SysInfo> Bulb<T> {
    /// Returns the bulb's system information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let sysinfo = bulb.sysinfo()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn sysinfo(&mut self) -> Result<T::Info> {
        self.device.sysinfo()
    }
}

impl Bulb<LB110> {
    /// Creates a new Bulb instance from the given local address.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// ```
    pub fn new<A>(host: A) -> Bulb<LB110>
    where
        A: Into<IpAddr>,
    {
        Bulb {
            device: LB110::new(host),
        }
    }

    /// Returns the software version of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let sw_ver = bulb.sw_ver()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn sw_ver(&mut self) -> Result<String> {
        self.device.sw_ver()
    }

    /// Returns the hardware version of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let hw_ver = bulb.hw_ver()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn hw_ver(&mut self) -> Result<String> {
        self.device.hw_ver()
    }

    /// Returns the model of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let model = bulb.model()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn model(&mut self) -> Result<String> {
        self.device.model()
    }

    /// Returns the name (alias) of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let alias = bulb.alias()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn alias(&mut self) -> Result<String> {
        self.device.alias()
    }

    /// Returns the mac address of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let mac_address = bulb.mac_address()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn mac_address(&mut self) -> Result<String> {
        self.device.mac_address()
    }

    /// Returns whether the bulb supports brightness changes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let is_dimmable = bulb.is_dimmable()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_dimmable(&mut self) -> Result<bool> {
        self.device.is_dimmable()
    }

    /// Returns whether the bulb supports color changes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let is_color = bulb.is_color()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_color(&mut self) -> Result<bool> {
        self.device.is_color()
    }

    /// Returns whether the bulb supports color temperature changes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let is_variable_color_temp = bulb.is_variable_color_temp()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_variable_color_temp(&mut self) -> Result<bool> {
        self.device.is_variable_color_temp()
    }

    /// Returns the Wi-Fi signal strength (rssi) of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let rssi = bulb.rssi()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn rssi(&mut self) -> Result<i64> {
        self.device.rssi()
    }

    /// Returns whether the device is currently switched on.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let is_on = bulb.is_on()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_on(&mut self) -> Result<bool> {
        self.device.is_on()
    }

    /// Returns the current HSV (Hue, Saturation, Value) state of the bulb.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let hsv = bulb.hsv()?;
    ///
    /// let hue = hsv.hue();                // degrees (0-360)
    /// let saturation = hsv.saturation();  // % (0-100)
    /// let brightness = hsv.value();       // % (0-100)
    /// # Ok(())
    /// # }
    /// ```
    pub fn hsv(&mut self) -> Result<HSV> {
        self.device.hsv()
    }

    /// Sets HSV (Hue, Saturation, Value) state of the bulb.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// if let Err(e) = bulb.set_hsv(270, 55, 90) {
    ///     eprintln!("error setting hsv: {}", e);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_hsv(&mut self, hue: u32, saturation: u32, value: u32) -> Result<()> {
        self.device.set_hsv(hue, saturation, value)
    }

    /// Returns whether the device supports `emeter` stats.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let has_emeter = bulb.has_emeter()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn has_emeter(&mut self) -> Result<bool> {
        self.device.has_emeter()
    }

    /// Sets the hue of the bulb, if the bulb supports color changes.
    /// Hue is color portion of the HSV model which is expressed as a
    /// number from 0 to 360 degrees.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// bulb.set_hue(140)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_hue(&mut self, hue: u32) -> Result<()> {
        self.device.set_hue(hue)
    }

    /// Returns the hue value (expressed as a number from 0 to 360 degrees)
    /// of the bulb, if the bulb supports color changes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// println!("hue: {}", bulb.hue()?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn hue(&mut self) -> Result<u32> {
        self.device.hue()
    }

    /// Sets the % saturation of the bulb, if the bulb supports color changes.
    /// Saturation determines the amount of gray in a particular color and is
    /// expressed as a number from 0 to 100 percent.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// bulb.set_saturation(70)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_saturation(&mut self, saturation: u32) -> Result<()> {
        self.device.set_saturation(saturation)
    }

    /// Returns the current % saturation of the bulb, if the bulb supports
    /// color changes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// println!("% saturation: {}", bulb.saturation()?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn saturation(&mut self) -> Result<u32> {
        self.device.saturation()
    }

    /// Sets the % brightness of the bulb, if the bulb supports brightness changes.
    /// Brightness determines the intensity of the color and is expressed
    /// as a number from 0 to 100 percent.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// bulb.set_brightness(30)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_brightness(&mut self, brightness: u32) -> Result<()> {
        self.device.set_brightness(brightness)
    }

    /// Returns the current % brightness of the bulb, if the bulb supports
    /// brightness changes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// println!("% brightness: {}", bulb.brightness()?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn brightness(&mut self) -> Result<u32> {
        self.device.brightness()
    }

    /// Sets the color temperature of the bulb, if the bulb supports color
    /// changes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// bulb.set_color_temp(2400)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_color_temp(&mut self, color_temp: u32) -> Result<()> {
        self.device.set_color_temp(color_temp)
    }

    /// Returns the current color temperature of the bulb.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// println!("color temperature: {}", bulb.color_temp()?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn color_temp(&mut self) -> Result<u32> {
        self.device.color_temp()
    }
}
