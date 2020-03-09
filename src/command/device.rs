use crate::error::Result;

/// The `Device` trait represents devices that are capable of
/// performing basic device commands.
pub trait Device {
    /// Turns on the device.
    fn turn_on(&mut self) -> Result<()>;

    /// Turns off the device.
    fn turn_off(&mut self) -> Result<()>;
}
