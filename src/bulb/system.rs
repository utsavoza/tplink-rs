use crate::error::Result;
use crate::proto::Proto;

use serde_json::Value;

pub(super) struct System(String);

impl System {
    pub(super) fn new(namespace: Option<&str>) -> System {
        System(namespace.unwrap_or("smartlife.iot.common.system").into())
    }

    pub(super) fn reboot(&self, proto: &mut Proto, arg: Option<&Value>) -> Result<()> {
        proto.send(&self.0, "reboot", arg).map(|_| {})
    }

    pub(super) fn reset(&self, proto: &mut Proto, arg: Option<&Value>) -> Result<()> {
        proto.send(&self.0, "reset", arg).map(|_| {})
    }
}
