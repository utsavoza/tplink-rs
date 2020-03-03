use crate::error::Result;
use crate::proto::Proto;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    #[serde(rename = "smartlife.iot.smartbulb.lightingservice")]
    light_state: LightState,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct LightState {
    #[serde(alias = "get_light_state", alias = "transition_light_state")]
    pub(super) values: Map<String, Value>,
}

pub(super) struct Lighting(String);

impl Lighting {
    pub(super) fn new(namespace: Option<&str>) -> Lighting {
        Lighting(
            namespace
                .unwrap_or("smartlife.iot.smartbulb.lightingservice")
                .into(),
        )
    }

    pub(super) fn get_light_state(&self, proto: &mut Proto) -> Result<LightState> {
        proto.send(&self.0, "get_light_state", None).map(|res| {
            serde_json::from_slice::<Response>(&res)
                .unwrap()
                .light_state
        })
    }

    pub(super) fn set_light_state(
        &self,
        proto: &mut Proto,
        arg: Option<&Value>,
    ) -> Result<LightState> {
        proto
            .send(&self.0, "transition_light_state", arg)
            .map(|res| {
                serde_json::from_slice::<Response>(&res)
                    .unwrap()
                    .light_state
            })
    }
}
