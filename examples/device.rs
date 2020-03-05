//! `cargo run --example device`

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);

    bulb.turn_on()?;
    assert_eq!(bulb.is_on()?, true);

    bulb.turn_off()?;
    assert_eq!(bulb.is_on()?, false);

    println!("supports brightness: {}", bulb.is_dimmable()?);
    println!("supports color: {}", bulb.is_color()?);
    println!("supports color temp: {}", bulb.is_variable_color_temp()?);

    match bulb.hsv() {
        Ok((hue, saturation, value)) => {
            println!("hue: {}, saturation: {}, value: {}", hue, saturation, value)
        }
        Err(e) => println!("error: {}", e),
    }

    Ok(())
}
