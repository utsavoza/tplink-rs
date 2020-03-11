//! `cargo run --example sysinfo`

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let bulb = tplink::Bulb::new([192, 168, 1, 101]);
    let sysinfo = bulb.sysinfo()?;
    println!("bulb sysinfo: {}\n", sysinfo);

    let plug = tplink::Plug::new([192, 168, 1, 100]);
    let sysinfo = plug.sysinfo()?;
    println!("plug sysinfo: {}", sysinfo);

    Ok(())
}
