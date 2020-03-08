mod hs100;

pub use crate::plug::hs100::Location;

use crate::command::time::{DeviceTime, DeviceTimeZone};
use crate::command::{Device, Sys, SysInfo, Time};
use crate::error::Result;
use crate::plug::hs100::HS100;

use std::net::IpAddr;
use std::time::Duration;

pub struct Plug<T> {
    device: T,
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
    /// let plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let sw_ver = plug.sw_ver()?;
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
    /// let plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let hw_ver = plug.hw_ver()?;
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
    /// let plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let model = plug.model()?;
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
    /// let plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let alias = plug.alias()?;
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
    /// let plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let mac_address = plug.mac_address()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn mac_address(&self) -> Result<String> {
        self.device.mac_address()
    }

    /// Returns the Wi-Fi signal strength (rssi) of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let rssi = plug.rssi()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn rssi(&self) -> Result<i64> {
        self.device.rssi()
    }

    /// Returns the location of the device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tplink::plug::Location;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let Location { latitude, longitude } = plug.location()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn location(&self) -> Result<Location> {
        self.device.location()
    }

    /// Returns whether the device is currently switched on.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let is_on = plug.is_on()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_on(&self) -> Result<bool> {
        self.device.is_on()
    }

    /// Returns whether the device LED is currently switched on.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let plug = tplink::Plug::new([192, 168, 1, 100]);
    /// let is_led_on = plug.is_led_on()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_led_on(&self) -> Result<bool> {
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

impl<T: SysInfo> Plug<T> {
    pub fn sysinfo(&self) -> Result<T::Info> {
        self.device.sysinfo()
    }
}

impl<T: Sys> Plug<T> {
    pub fn reboot(&mut self, delay: Option<Duration>) -> Result<()> {
        self.device.reboot(delay)
    }

    pub fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()> {
        self.device.factory_reset(delay)
    }
}

impl<T: Device> Plug<T> {
    pub fn turn_on(&mut self) -> Result<()> {
        self.device.turn_on()
    }

    pub fn turn_off(&mut self) -> Result<()> {
        self.device.turn_off()
    }
}

impl<T: Time> Plug<T> {
    pub fn time(&self) -> Result<DeviceTime> {
        self.device.time()
    }

    pub fn timezone(&self) -> Result<DeviceTimeZone> {
        self.device.timezone()
    }
}
