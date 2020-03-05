use crate::error::Result;

pub trait SysInfo {
    type Info;

    fn sysinfo(&self) -> Result<Self::Info>;
}
