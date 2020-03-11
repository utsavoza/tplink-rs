//! `cargo run --example discover`

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for (ip, device) in tplink::discover()? {
        match device {
            tplink::DeviceKind::Plug(mut plug) => {
                println!("[{}] => {}", ip, plug.alias()?);

                plug.turn_on()?;
                assert_eq!(plug.is_on()?, true);

                plug.turn_off()?;
                assert_eq!(plug.is_on()?, false);
            }
            tplink::DeviceKind::Bulb(mut bulb) => {
                println!("[{}] => {}", ip, bulb.alias()?);

                bulb.turn_on()?;
                assert_eq!(bulb.is_on()?, true);

                if bulb.is_dimmable()? {
                    bulb.set_brightness(0)?;
                }
            }
            _ => println!("unknown device"),
        }
    }
    Ok(())
}
