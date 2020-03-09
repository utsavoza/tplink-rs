use crate::error::Result;
use crate::proto::{Proto, Request};

use serde::de::DeserializeOwned;
use serde::export::PhantomData;

/// The `SysInfo` trait represents devices that are capable of
/// returning their system information.
pub trait SysInfo {
    /// The type of system information returned by the device.
    type Info;

    /// Attempt to fetch the system information from the device.
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

impl<T: DeserializeOwned> SystemInfo<T> {
    pub(crate) fn get_sysinfo(&self, proto: &Proto) -> Result<T> {
        proto
            .send_request(&Request::from("system", "get_sysinfo", None))
            .map(|response| {
                serde_json::from_value(response).unwrap_or_else(|err| {
                    panic!(
                        "invalid response from host with address {}: {}",
                        proto.host(),
                        err
                    )
                })
            })
    }
}
