use crate::error::Result;
use crate::proto::Proto;

use serde::de::DeserializeOwned;
use serde::export::PhantomData;

pub trait SysInfo {
    type Info;

    fn sysinfo(&self) -> Result<Self::Info>;
}

pub(crate) struct SystemInfo<T> {
    _ghost: PhantomData<T>,
}

impl<T> SystemInfo<T> {
    pub(crate) fn new() -> SystemInfo<T> {
        SystemInfo {
            _ghost: PhantomData,
        }
    }
}

impl<T> SystemInfo<T>
where
    T: DeserializeOwned,
{
    pub(crate) fn get_sysinfo(&self, proto: &Proto) -> Result<T> {
        proto
            .send_command("system", "get_sysinfo", None)
            .map(|mut value| serde_json::from_value(value["system"]["get_sysinfo"].take()).unwrap())
    }
}
