//! `cargo run --example plug`

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut plug = tplink::Plug::new([192, 168, 1, 100]);

    plug.turn_on()?;
    assert_eq!(plug.is_on()?, true);

    plug.turn_off()?;
    assert_eq!(plug.is_on()?, false);

    plug.turn_off_led()?;
    assert_eq!(plug.is_led_on()?, false);

    plug.turn_on_led()?;
    assert_eq!(plug.is_led_on()?, true);

    println!("location: {}", plug.location()?);
    println!("alias: {}", plug.alias()?);
    println!("model: {}", plug.model()?);
    println!("time: {}", plug.time()?);
    println!("has emeter: {}", plug.has_emeter()?);

    Ok(())
}
