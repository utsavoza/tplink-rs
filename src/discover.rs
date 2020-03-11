use crate::bulb::LB110;
use crate::error::Result;
use crate::plug::HS100;
use crate::{proto, Bulb, Plug};

use serde_json::{json, Value};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;

/// Types of TP-Link Wi-Fi Smart Home Devices.
pub enum DeviceKind {
    /// TP-Link Smart Wi-Fi Plug.
    Plug(Box<Plug<HS100>>),
    /// TP-Link Smart Wi-Fi Bulb.
    Bulb(Box<Bulb<LB110>>),
    /// TP-Link Smart Wi-Fi Power Strip
    Strip,
    /// Encompasses any other TP-Link devices that
    /// are not recognised by the library.
    Unknown,
}

/// Discover existing TP-Link Smart Home devices on the network.
///
/// # Examples
///
/// ```no_run
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     for (ip, device) in tplink::discover()? {
///         match device {
///             tplink::DeviceKind::Plug(mut plug) => {
///                 // .. do something with the plug
///             },
///             tplink::DeviceKind::Bulb(mut bulb) => {
///                 // .. do something with the bulb
///             },
///             _ => println!("unrecognised device on the network: {}", ip),
///         }
///     }
///     Ok(())
/// }
/// ```
pub fn discover() -> Result<HashMap<IpAddr, DeviceKind>> {
    let query = json!({
        "system": {"get_sysinfo": {}},
        "emeter": {"get_realtime": {}},
        "smartlife.iot.dimmer": {"get_dimmer_parameters": {}},
        "smartlife.iot.common.emeter": {"get_realtime": {}},
        "smartlife.iot.smartbulb.lightingservice": {"get_light_state": {}},
    });
    let request = serde_json::to_vec(&query).unwrap();
    let proto = proto::Builder::new([255, 255, 255, 255])
        .broadcast(true)
        .read_timeout(Duration::from_secs(3))
        .write_timeout(Duration::from_secs(3))
        .offline_tolerance(3)
        .build();
    let responses = proto.discover(&request)?;

    let mut devices = HashMap::new();
    for (ip, response) in responses {
        let value = serde_json::from_slice::<Value>(&response).unwrap();
        let device = device_from(ip, &value)?;
        devices.entry(ip).or_insert(device);
    }

    Ok(devices)
}

fn device_from(host: IpAddr, value: &Value) -> Result<DeviceKind> {
    let (device_type, sysinfo) = {
        if value.get("system").is_some() && value["system"].get("get_sysinfo").is_some() {
            let sysinfo = &value["system"]["get_sysinfo"];
            if sysinfo.get("type").is_some() {
                (sysinfo["type"].to_string().to_lowercase(), sysinfo)
            } else if sysinfo.get("mic_type").is_some() {
                (sysinfo["mic_type"].to_string().to_lowercase(), sysinfo)
            } else {
                panic!("invalid discovery response received")
            }
        } else {
            panic!("invalid discovery response received")
        }
    };

    if device_type.contains("plug") && sysinfo.get("children").is_some() {
        Ok(DeviceKind::Strip)
    } else if device_type.contains("plug") {
        Ok(DeviceKind::Plug(Box::from(Plug::new(host))))
    } else if device_type.contains("bulb") {
        Ok(DeviceKind::Bulb(Box::from(Bulb::new(host))))
    } else {
        Ok(DeviceKind::Unknown)
    }
}
