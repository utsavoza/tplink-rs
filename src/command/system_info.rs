use crate::error::Result;

pub trait SystemInfo {
    type Info;

    fn sysinfo(&mut self) -> Result<Self::Info>;
}
