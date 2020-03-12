//! `cargo run --example discover`

use tplink::DeviceKind;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let devices = tplink::discover()?;

    for (ip, device) in devices {
        match device {
            DeviceKind::Plug(mut plug) => {
                println!("[{}] => {}", ip, plug.alias()?);

                if plug.is_on()? {
                    plug.turn_off()?;
                    assert_eq!(plug.is_on()?, false);
                }
            }
            DeviceKind::Bulb(mut bulb) => {
                println!("[{}] => {}", ip, bulb.alias()?);

                bulb.turn_on()?;
                assert_eq!(bulb.is_on()?, true);

                if bulb.is_on()? && bulb.is_dimmable()? {
                    bulb.set_brightness(100)?;
                    assert_eq!(bulb.brightness()?, 100);
                }
            }
            _ => eprintln!("unrecognised device found on the network: {}", ip),
        }
    }

    Ok(())
}
