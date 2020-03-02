use crate::error::Result;

pub trait Device {
    fn turn_on(&mut self) -> Result<()>;
    fn turn_off(&mut self) -> Result<()>;
}
