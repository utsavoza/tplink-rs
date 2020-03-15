//! `cargo run --example discover`

use tplink::DeviceKind;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let devices = tplink::discover()?;

    for (ip, device) in devices {
        match device {
            DeviceKind::Plug(mut plug) => {
                println!("[{}] => {}", ip, plug.alias()?);

                plug.turn_off()?;
                assert_eq!(plug.is_on()?, false);
            }
            DeviceKind::Bulb(mut bulb) => {
                println!("[{}] => {}", ip, bulb.alias()?);

                bulb.set_brightness(0)?;
                assert_eq!(bulb.brightness()?, 0);
            }
            _ => eprintln!("unrecognised device found on the network: {}", ip),
        }
    }

    Ok(())
}
