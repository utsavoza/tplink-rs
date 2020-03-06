//! `cargo run --example sysinfo`

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let bulb = tplink::Bulb::new([192, 168, 1, 101]);

    let sysinfo = bulb.sysinfo()?;
    println!("sysinfo: {}", sysinfo);

    let is_dimmable = sysinfo.is_dimmable();
    println!("is_dimmable: {}", is_dimmable);

    let is_color = sysinfo.is_color();
    println!("is_color: {}", is_color);

    let is_variable_color_temp = sysinfo.is_variable_color_temp();
    println!("is_variable_color_temp: {}", is_variable_color_temp);

    let sw_ver = sysinfo.sw_ver();
    println!("sw_ver: {}", sw_ver);

    let hw_ver = sysinfo.hw_ver();
    println!("hw_ver: {}", hw_ver);

    let alias = sysinfo.alias();
    println!("alias: {}", alias);

    let model = sysinfo.model();
    println!("model: {}", model);

    let mac_address = sysinfo.mac_address();
    println!("mac_address: {}", mac_address);

    Ok(())
}
