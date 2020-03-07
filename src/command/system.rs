use crate::error::Result;
use crate::proto::Proto;

use serde_json::json;
use std::time::Duration;

pub trait Sys {
    fn reboot(&mut self, delay: Option<Duration>) -> Result<()>;
    fn factory_reset(&mut self, delay: Option<Duration>) -> Result<()>;
}

pub(crate) struct System {
    ns: String,
}

impl System {
    pub(crate) fn new(ns: &str) -> System {
        System { ns: ns.into() }
    }

    pub(crate) fn reboot(&self, proto: &Proto, delay: Option<Duration>) -> Result<()> {
        let delay_in_secs = delay.map_or(1, |duration| duration.as_secs());
        let res =
            proto.send_command(&self.ns, "reboot", Some(&json!({ "delay": delay_in_secs })))?;
        log::debug!("{:?}", res);
        Ok(())
    }

    pub(crate) fn factory_reset(&self, proto: &Proto, delay: Option<Duration>) -> Result<()> {
        let delay_in_secs = delay.map_or(1, |duration| duration.as_secs());
        let res =
            proto.send_command(&self.ns, "reset", Some(&json!({ "delay": delay_in_secs })))?;
        log::debug!("{:?}", res);
        Ok(())
    }
}
