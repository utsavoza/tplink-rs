use crate::error::Result;
use std::time::Duration;

pub trait System {
    fn reboot(&mut self, delay: Option<Duration>) -> Result<()>;
    fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()>;
}
