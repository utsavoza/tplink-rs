use crate::error::Result;
use crate::proto::Proto;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(super) struct Lighting {
    ns: String,
}

impl Lighting {
    pub(super) fn new(ns: &str) -> Lighting {
        Lighting { ns: ns.into() }
    }

    pub(super) fn get_light_state(&self, proto: &Proto) -> Result<LightState> {
        proto
            .send_command(&self.ns, "get_light_state", None)
            .map(|mut value| {
                serde_json::from_value(value[&self.ns]["get_light_state"].take()).unwrap()
            })
    }

    pub(super) fn set_light_state(&self, proto: &Proto, arg: Option<&Value>) -> Result<LightState> {
        proto
            .send_command(&self.ns, "transition_light_state", arg)
            .map(|mut value| {
                serde_json::from_value(value[&self.ns]["transition_light_state"].take()).unwrap()
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
    mode: Option<String>,
    hue: u64,
    saturation: u64,
    color_temp: u64,
    brightness: u64,
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
