use crate::error::Result;

pub trait SystemInfo {
    type Info;

    fn sys_info(&mut self) -> Result<Self::Info>;
}
