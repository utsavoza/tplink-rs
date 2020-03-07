//! `cargo run --example bulb`

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut bulb = tplink::Bulb::new([192, 168, 1, 100]);

    bulb.turn_on()?;
    assert_eq!(bulb.is_on()?, true);

    bulb.turn_off()?;
    assert_eq!(bulb.is_on()?, false);

    println!("supports brightness: {}", bulb.is_dimmable()?);
    println!("supports color: {}", bulb.is_color()?);
    println!("supports color temp: {}", bulb.is_variable_color_temp()?);
    println!("has emeter: {}", bulb.has_emeter()?);
    println!("time: {}", bulb.time()?);

    match bulb.hsv() {
        Ok(hsv) => println!(
            "hue: {}, saturation: {}, value: {}",
            hsv.hue(),
            hsv.saturation(),
            hsv.value()
        ),
        Err(e) => println!("error: {}", e),
    }

    Ok(())
}
