use crate::cache::ResponseCache;
use crate::error::Result;
use crate::proto::{Proto, Request};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(super) struct Lighting {
    ns: String,
}

impl Lighting {
    pub(super) fn new(ns: &str) -> Lighting {
        Lighting {
            ns: String::from(ns),
        }
    }

    pub(super) fn get_light_state(
        &self,
        proto: &Proto,
        cache: &mut ResponseCache,
    ) -> Result<LightState> {
        let request = Request::new(&self.ns, "get_light_state", None);

        let response = if let Some(cache) = cache {
            cache.get_or_insert_with(request, |r| proto.send_request(r))?
        } else {
            proto.send_request(&request)?
        };

        log::trace!("({}) {:?}", self.ns, response);

        Ok(serde_json::from_value(response).unwrap_or_else(|err| {
            panic!(
                "invalid response from host with address {}: {}",
                proto.host(),
                err
            )
        }))
    }

    pub(super) fn set_light_state(
        &self,
        proto: &Proto,
        cache: &mut ResponseCache,
        arg: Option<Value>,
    ) -> Result<()> {
        if let Some(c) = cache {
            c.retain(|k, _| k.target != self.ns)
        }

        let response = proto
            .send_request(&Request::new(&self.ns, "transition_light_state", arg))
            .map(|response| {
                serde_json::from_value::<LightState>(response).unwrap_or_else(|err| {
                    panic!(
                        "invalid response from host with address {}: {}",
                        proto.host(),
                        err
                    )
                })
            })?;

        log::trace!("({}) {:?}", self.ns, response);

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct LightState {
    on_off: u64,
    #[serde(flatten)]
    hsv: Option<HSV>,
    dft_on_state: Option<HSV>,
}

impl LightState {
    pub(super) fn is_on(&self) -> bool {
        self.on_off == 1
    }

    pub(super) fn hsv(&self) -> HSV {
        if self.on_off == 1 {
            self.hsv.as_ref().unwrap().clone()
        } else {
            self.dft_on_state.as_ref().unwrap().clone()
        }
    }
}

/// The HSV (Hue, Saturation, Value) state of the bulb.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HSV {
    hue: u32,
    saturation: u32,
    brightness: u32,
    color_temp: u32,
    mode: Option<String>,
}

impl HSV {
    /// Returns the `hue` (color portion) of the HSV model, expressed
    /// as a number from 0 to 360 degrees.
    pub fn hue(&self) -> u32 {
        self.hue
    }

    /// Returns the `saturation` (amount of gray in particular color)
    /// of the HSV model, expressed as a number from 0 to 100 percent.
    pub fn saturation(&self) -> u32 {
        self.saturation
    }

    /// Returns the `value` or `brightness` (intensity of the color)
    /// of the HSV model, expressed as a number from 0 to 100 percent.
    pub fn value(&self) -> u32 {
        self.brightness
    }

    pub fn color_temp(&self) -> u32 {
        self.color_temp
    }
}
