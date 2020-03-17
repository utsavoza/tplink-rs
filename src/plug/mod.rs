mod hs100;

pub use self::hs100::{Location, HS100};
use crate::command::cloud::{Cloud, CloudInfo};
use crate::command::device::Device;
use crate::command::sys::Sys;
use crate::command::sysinfo::SysInfo;
use crate::command::time::{DeviceTime, DeviceTimeZone, Time};
use crate::command::wlan::{AccessPoint, Wlan};
use crate::error::Result;

use std::net::IpAddr;
use std::time::Duration;

/// A TP-Link Smart Plug.
///
/// # Examples
///
/// ```no_run
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut plug = tplink::Plug::new([192, 168, 1, 100]);
///
///     plug.turn_on()?;
///     assert_eq!(plug.is_on()?, true);
///
///     plug.turn_off()?;
///     assert_eq!(plug.is_on()?, false);
///
///     Ok(())
/// }
/// ```
pub struct Plug<T> {
    device: T,
}

impl<T: Device> Plug<T> {
    /// Turns on the plug.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// plug.turn_on()?;
    /// assert_eq!(plug.is_on()?, true);
    /// # Ok(())
    /// # }
    /// ```
    pub fn turn_on(&mut self) -> Result<()> {
        self.device.turn_on()
    }

    /// Turns off the plug.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// plug.turn_off()?;
    /// assert_eq!(plug.is_on()?, false);
    /// # Ok(())
    /// # }
    /// ```
    pub fn turn_off(&mut self) -> Result<()> {
        self.device.turn_off()
    }
}

impl<T: Sys> Plug<T> {
    /// Reboots the plug after the given duration. In case when the delay
    /// duration is not provided, the plug is set to reboot after a default
    /// delay of 1 second.
    ///
    /// # Examples
    /// Reboots the plug after a delay of 3 seconds.
    ///
    /// ```no_run
    /// use std::time::Duration;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// plug.reboot(Some(Duration::from_secs(3)))?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Reboots the plug after 1 second.
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// plug.reboot(None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.device.reboot(delay)
    }

    /// Factory resets the plug after the given duration. In case when the delay
    /// duration is not provided, the plug is set to reset after a default delay
    /// of 1 second.
    ///
    /// # Examples
    /// Factory resets the plug after a delay of 3 seconds.
    ///
    /// ```no_run
    /// use std::time::Duration;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// plug.factory_reset(Some(Duration::from_secs(3)))?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Factory resets the plug after 1 second.
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// plug.factory_reset(None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.device.factory_reset(delay)
    }
}

impl<T: Time> Plug<T> {
    /// Returns the current date and time of the device without the timezone.
    /// To get the device timezone, use [`timezone`] method.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let time = plug.time()?; // e.g. `2020-04-09 22:32:01`
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
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let timezone = plug.timezone()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn timezone(&mut self) -> Result<DeviceTimeZone> {
        self.device.timezone()
    }
}

impl<T: Cloud> Plug<T> {
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

impl<T: Wlan> Plug<T> {
    pub fn get_scan_info(
        &mut self,
        refresh: bool,
        timeout: Option<Duration>,
    ) -> Result<Vec<AccessPoint>> {
        self.device.get_scan_info(refresh, timeout)
    }
}

impl<T: SysInfo> Plug<T> {
    /// Returns the plug's system information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let sysinfo = plug.sysinfo()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn sysinfo(&mut self) -> Result<T::Info> {
        self.device.sysinfo()
    }
}

impl Plug<HS100> {
    /// Creates a new Plug instance from the given local address.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let plug = tplink::Plug::new([192, 168, 1, 100]);
    /// ```
    pub fn new<A>(host: A) -> Plug<HS100>
    where
        A: Into<IpAddr>,
    {
        Plug {
            device: HS100::new(host),
        }
    }

    /// Returns the software version of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let sw_ver = plug.sw_ver()?;
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
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let hw_ver = plug.hw_ver()?;
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
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let model = plug.model()?;
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
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let alias = plug.alias()?;
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
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let mac_address = plug.mac_address()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn mac_address(&mut self) -> Result<String> {
        self.device.mac_address()
    }

    /// Returns the Wi-Fi signal strength (rssi) of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let rssi = plug.rssi()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn rssi(&mut self) -> Result<i64> {
        self.device.rssi()
    }

    /// Returns the location of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let location = plug.location()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn location(&mut self) -> Result<Location> {
        self.device.location()
    }

    /// Returns whether the device is currently switched on.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let is_on = plug.is_on()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_on(&mut self) -> Result<bool> {
        self.device.is_on()
    }

    /// Returns whether the device LED is currently switched on.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let is_led_on = plug.is_led_on()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_led_on(&mut self) -> Result<bool> {
        self.device.is_led_on()
    }

    /// Turns on the device's LED.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// plug.turn_on_led()?;
    /// assert_eq!(plug.is_led_on()?, true);
    /// # Ok(())
    /// # }
    /// ```
    pub fn turn_on_led(&mut self) -> Result<()> {
        self.device.turn_on_led()
    }

    /// Turns off the device's LED.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    /// plug.turn_off_led()?;
    /// assert_eq!(plug.is_led_on()?, false);
    /// # Ok(())
    /// # }
    /// ```
    pub fn turn_off_led(&mut self) -> Result<()> {
        self.device.turn_off_led()
    }
}
