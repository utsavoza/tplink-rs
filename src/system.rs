use crate::error::Result;

pub trait System {
    type SystemInfo;

    fn sys_info(&self) -> Result<Self::SystemInfo>;
}
