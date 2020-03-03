//! `cargo run --example bulb`

fn main() {
    env_logger::init();

    let mut bulb = tplink::Bulb::new([192, 168, 1, 101]);

    let sysinfo = bulb.sysinfo().unwrap();
    println!("{:?}", sysinfo);

    let is_dimmable = sysinfo.is_dimmable().unwrap();
    println!("is_dimmable: {}", is_dimmable);

    let is_color = sysinfo.is_color().unwrap();
    println!("is_color: {}", is_color);

    let is_variable_color_temp = sysinfo.is_variable_color_temp().unwrap();
    println!("is_variable_color_temp: {}", is_variable_color_temp);

    let sw_ver = sysinfo.sw_ver().unwrap();
    println!("sw_ver: {}", sw_ver);

    let hw_ver = sysinfo.hw_ver().unwrap();
    println!("hw_ver: {}", hw_ver);

    let device_id = sysinfo.device_id().unwrap();
    println!("device_id: {}", device_id);

    let alias = sysinfo.alias().unwrap();
    println!("alias: {}", alias);

    let model = sysinfo.model().unwrap();
    println!("model: {}", model);

    let device_type = sysinfo.device_type().unwrap();
    println!("device_type: {}", device_type);

    let mac_address = sysinfo.mac_address().unwrap();
    println!("mac_address: {}", mac_address);
}
