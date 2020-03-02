use crate::error::Result;
use std::time::Duration;

pub trait System {
    type SystemInfo;

    fn sys_info(&mut self) -> Result<Self::SystemInfo>;
    fn reboot(&mut self, delay: Option<Duration>) -> Result<()>;
    fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()>;
}
