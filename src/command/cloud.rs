use crate::cache::ResponseCache;
use crate::error::Result;
use crate::proto::{Proto, Request};

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;
use std::rc::Rc;

pub trait Cloud {
    fn get_cloud_info(&mut self) -> Result<CloudInfo>;
    fn bind(&mut self, username: &str, password: &str) -> Result<()>;
    fn unbind(&mut self) -> Result<()>;
    fn get_firmware_list(&mut self) -> Result<Vec<String>>;
    fn set_server_url(&mut self, url: &str) -> Result<()>;
}

pub(crate) struct CloudSettings {
    ns: String,
    proto: Rc<Proto>,
    cache: Rc<ResponseCache>,
}

impl CloudSettings {
    pub(crate) fn new(ns: &str, proto: Rc<Proto>, cache: Rc<ResponseCache>) -> CloudSettings {
        CloudSettings {
            ns: String::from(ns),
            proto,
            cache,
        }
    }

    pub(crate) fn get_info(&self) -> Result<CloudInfo> {
        let request = Request::new(&self.ns, "get_info", None);

        let response = if let Some(cache) = self.cache.as_ref() {
            cache
                .borrow_mut()
                .try_get_or_insert_with(request, |r| self.proto.send_request(r))?
        } else {
            self.proto.send_request(&request)?
        };

        log::trace!("{:?}", response);

        Ok(serde_json::from_value(response).unwrap_or_else(|err| {
            panic!(
                "invalid response from host with address {}: {}",
                self.proto.host(),
                err
            )
        }))
    }

    pub(crate) fn bind(&self, username: &str, password: &str) -> Result<()> {
        if let Some(cache) = self.cache.as_ref() {
            cache.borrow_mut().retain(|k, _| k.target != self.ns)
        }

        let response = self.proto.send_request(&Request::new(
            &self.ns,
            "bind",
            Some(json!({ "username": username, "password": password })),
        ))?;

        log::trace!("{:?}", response);

        Ok(())
    }

    pub(crate) fn unbind(&self) -> Result<()> {
        if let Some(cache) = self.cache.as_ref() {
            cache.borrow_mut().retain(|k, _| k.target != self.ns)
        }

        let response = self
            .proto
            .send_request(&Request::new(&self.ns, "unbind", None))?;

        log::trace!("{:?}", response);

        Ok(())
    }

    pub(crate) fn get_firmware_list(&self) -> Result<Vec<String>> {
        let request = Request::new(&self.ns, "get_intl_fw_list", None);

        let response = if let Some(cache) = self.cache.as_ref() {
            cache
                .borrow_mut()
                .try_get_or_insert_with(request, |r| self.proto.send_request(r))?
        } else {
            self.proto.send_request(&request)?
        };

        log::trace!("{:?}", response);

        let fw_list = serde_json::from_value::<FirmwareList>(response)
            .map(|response| response.fw_list)
            .unwrap_or_else(|err| {
                panic!(
                    "invalid response from host with address {}: {}",
                    self.proto.host(),
                    err
                )
            });

        Ok(fw_list)
    }

    pub(crate) fn set_server_url(&self, url: &str) -> Result<()> {
        if let Some(cache) = self.cache.as_ref() {
            cache.borrow_mut().retain(|k, _| k.target != self.ns)
        }

        let response = self.proto.send_request(&Request::new(
            &self.ns,
            "set_server_url",
            Some(json!({ "server": url })),
        ))?;

        log::trace!("{:?}", response);

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct FirmwareList {
    fw_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudInfo {
    binded: u32,
    cld_connection: u32,
    #[serde(alias = "fwDlPage")]
    fw_dl_page: String,
    #[serde(alias = "fwNotifyType")]
    fw_notify_type: u32,
    #[serde(alias = "illegalType")]
    illegal_type: u32,
    server: String,
    #[serde(alias = "stopConnect")]
    stop_connect: u32,
    #[serde(alias = "tcspInfo")]
    tcsp_info: String,
    #[serde(alias = "tcspStatus")]
    tcsp_status: u32,
    username: String,
}

impl CloudInfo {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    pub fn bounded(&self) -> bool {
        self.binded == 1
    }
}

impl fmt::Display for CloudInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
