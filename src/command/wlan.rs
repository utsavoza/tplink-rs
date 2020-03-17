use crate::error::Result;
use crate::proto::{Proto, Request};

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;

pub trait Wlan {
    fn get_scan_info(
        &mut self,
        refresh: bool,
        timeout: Option<Duration>,
    ) -> Result<Vec<AccessPoint>>;
}

pub(crate) struct Netif {
    ns: String,
}

impl Netif {
    pub(crate) fn new() -> Netif {
        Netif {
            ns: String::from("netif"),
        }
    }

    pub(crate) fn get_scan_info(
        &self,
        proto: &Proto,
        refresh: bool,
        timeout: Option<Duration>,
    ) -> Result<Vec<AccessPoint>> {
        let refresh = if refresh { 1 } else { 0 };
        // Note: If scan timeout is greater than proto's read timeout,
        // the method returns with an ErrorKind::WouldBlock error.
        let timeout = timeout.map_or(
            proto.read_timeout().map_or(3, |to| to.as_secs()),
            |duration| duration.as_secs(),
        );

        let response = proto.send_request(&Request::new(
            &self.ns,
            "get_scaninfo",
            Some(json!({ "refresh": refresh, "timeout": timeout })),
        ))?;

        log::trace!("{:?}", response);

        Ok(serde_json::from_value::<AccessPointList>(response)
            .map(|response| response.ap_list)
            .unwrap_or_else(|err| {
                panic!(
                    "invalid response from host with address {}: {}",
                    proto.host(),
                    err
                )
            }))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AccessPointList {
    ap_list: Vec<AccessPoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessPoint {
    ssid: String,
    key_type: u32,
}

impl AccessPoint {
    pub fn ssid(&self) -> &str {
        &self.ssid
    }

    pub fn key_type(&self) -> u32 {
        self.key_type
    }
}
