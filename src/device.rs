use crate::error::Result;

pub trait Device {
    fn turn_on(&self) -> Result<()>;
    fn turn_off(&self) -> Result<()>;
}
