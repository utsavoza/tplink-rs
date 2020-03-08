mod lb110;
mod lighting;

use crate::bulb::lb110::LB110;
use crate::bulb::lighting::HSV;
use crate::command::time::{DeviceTime, DeviceTimeZone};
use crate::command::{Device, Sys, SysInfo, Time};
use crate::error::Result;

use std::net::IpAddr;
use std::time::Duration;

pub struct Bulb<T> {
    device: T,
}

impl<T: Device> Bulb<T> {
    pub fn turn_on(&mut self) -> Result<()> {
        self.device.turn_on()
    }

    pub fn turn_off(&mut self) -> Result<()> {
        self.device.turn_off()
    }
}

impl<T: Sys> Bulb<T> {
    pub fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.device.reboot(delay)
    }

    pub fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.device.factory_reset(delay)
    }
}

impl<T: Time> Bulb<T> {
    pub fn time(&self) -> Result<DeviceTime> {
        self.device.time()
    }

    pub fn timezone(&self) -> Result<DeviceTimeZone> {
        self.device.timezone()
    }
}

impl<T: SysInfo> Bulb<T> {
    pub fn sysinfo(&self) -> Result<T::Info> {
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
    /// let bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let sw_ver = bulb.sw_ver()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn sw_ver(&self) -> Result<String> {
        self.device.sw_ver()
    }

    /// Returns the hardware version of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let hw_ver = bulb.hw_ver()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn hw_ver(&self) -> Result<String> {
        self.device.hw_ver()
    }

    /// Returns the model of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let model = bulb.model()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn model(&self) -> Result<String> {
        self.device.model()
    }

    /// Returns the name (alias) of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let alias = bulb.alias()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn alias(&self) -> Result<String> {
        self.device.alias()
    }

    /// Returns the mac address of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let mac_address = bulb.mac_address()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn mac_address(&self) -> Result<String> {
        self.device.mac_address()
    }

    /// Returns whether the bulb supports brightness changes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let is_dimmable = bulb.is_dimmable()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_dimmable(&self) -> Result<bool> {
        self.device.is_dimmable()
    }

    /// Returns whether the bulb supports color changes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let is_color = bulb.is_color()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_color(&self) -> Result<bool> {
        self.device.is_color()
    }

    /// Returns whether the bulb supports color temperature changes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let is_variable_color_temp = bulb.is_variable_color_temp()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_variable_color_temp(&self) -> Result<bool> {
        self.device.is_variable_color_temp()
    }

    /// Returns the Wi-Fi signal strength (rssi) of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let rssi = bulb.rssi()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn rssi(&self) -> Result<i64> {
        self.device.rssi()
    }

    /// Returns whether the device is currently switched on.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let is_on = bulb.is_on()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_on(&self) -> Result<bool> {
        self.device.is_on()
    }

    /// Returns the current HSV (Hue, Saturation, Value) state of the bulb.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let hsv = bulb.hsv()?;
    ///
    /// let hue = hsv.hue();                // degrees (0-360)
    /// let saturation = hsv.saturation();  // % (0-100)
    /// let brightness = hsv.value();       // % (0-100)
    /// # Ok(())
    /// # }
    /// ```
    pub fn hsv(&self) -> Result<HSV> {
        self.device.hsv()
    }

    /// Returns whether the device supports `emeter` stats.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bulb = tplink::Bulb::new([192, 168, 1, 101]);
    /// let has_emeter = bulb.has_emeter()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn has_emeter(&self) -> Result<bool> {
        Ok(true)
    }
}
