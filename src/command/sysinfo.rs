use crate::cache::ResponseCache;
use crate::error::Result;
use crate::proto::{Proto, Request};

use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use std::rc::Rc;

/// The `SysInfo` trait represents devices that are capable of
/// returning their system information.
pub trait SysInfo {
    /// The type of system information returned by the device.
    type Info;

    /// Attempts to fetch the system information from the device.
    fn sysinfo(&mut self) -> Result<Self::Info>;
}

pub(crate) struct SystemInfo<T> {
    proto: Rc<Proto>,
    cache: Rc<ResponseCache>,
    _ghost: PhantomData<T>,
}

impl<T> SystemInfo<T> {
    pub(crate) fn new(proto: Rc<Proto>, cache: Rc<ResponseCache>) -> SystemInfo<T> {
        SystemInfo {
            proto,
            cache,
            _ghost: PhantomData,
        }
    }
}

impl<T: DeserializeOwned> SystemInfo<T> {
    pub(crate) fn get_sysinfo(&self) -> Result<T> {
        let request = Request::new("system", "get_sysinfo", None);

        let response = if let Some(cache) = self.cache.as_ref() {
            cache
                .borrow_mut()
                .get_or_insert_with(request, |r| self.proto.send_request(r))?
        } else {
            self.proto.send_request(&request)?
        };

        log::trace!("(system) {:?}", response);

        Ok(serde_json::from_value(response).unwrap_or_else(|err| {
            panic!(
                "invalid response from host with address {}: {}",
                self.proto.host(),
                err
            )
        }))
    }
}
