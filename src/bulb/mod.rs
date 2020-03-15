mod lb110;
mod lighting;

pub use self::lb110::LB110;

use crate::bulb::lighting::HSV;
use crate::command::time::{DeviceTime, DeviceTimeZone};
use crate::command::{Device, Sys, SysInfo, Time};
use crate::error::Result;

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
        Ok(true)
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
