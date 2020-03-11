use crate::cache::Cache;
use crate::error::Result;
use crate::proto::{Proto, Request};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefMut;

pub(super) struct Lighting {
    ns: String,
}

impl Lighting {
    pub(super) fn new(ns: &str) -> Lighting {
        Lighting { ns: ns.into() }
    }

    pub(super) fn get_light_state(
        &self,
        proto: &Proto,
        cache: &mut RefMut<Cache<Request, Value>>,
    ) -> Result<LightState> {
        let request = Request::new(&self.ns, "get_light_state", None);
        let response = match cache.get(&request) {
            Some(value) => value.to_owned(),
            None => {
                let value = proto.send_request(&request)?;
                cache.insert(request, value.to_owned());
                value
            }
        };
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
        arg: Option<Value>,
        cache: &mut RefMut<Cache<Request, Value>>,
    ) -> Result<LightState> {
        cache.retain(|k, _| k.target != self.ns);
        proto
            .send_request(&Request::new(&self.ns, "transition_light_state", arg))
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
    hue: u64,
    saturation: u64,
    brightness: u64,
    color_temp: u64,
    mode: Option<String>,
}

impl HSV {
    /// Returns the `hue` (color portion) of the HSV model, expressed
    /// as a number from 0 to 360 degrees.
    pub fn hue(&self) -> u64 {
        self.hue
    }

    /// Returns the `saturation` (amount of gray in particular color)
    /// of the HSV model, expressed as a number from 0 to 100 percent.
    pub fn saturation(&self) -> u64 {
        self.saturation
    }

    /// Returns the `value` or `brightness` (intensity of the color)
    /// of the HSV model, expressed as a number from 0 to 100 percent.
    pub fn value(&self) -> u64 {
        self.brightness
    }
}
