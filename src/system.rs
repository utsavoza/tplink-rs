use crate::error::Result;

pub trait System {
    type SystemInfo;

    fn sys_info(&mut self) -> Result<Self::SystemInfo>;

    fn reboot(&mut self) -> Result<()>;
}
