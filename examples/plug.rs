//! `cargo run --example plug`

fn main() {
    env_logger::init();

    let mut plug = tplink::Plug::new([192, 168, 1, 100]);

    let sys_info = plug.sys_info().unwrap();
    println!("sys_info: {}", sys_info);

    let sw_ver = sys_info.sw_ver().unwrap();
    println!("sw_ver: {}", sw_ver);

    let hw_ver = sys_info.hw_ver().unwrap();
    println!("hw_ver: {}", hw_ver);

    let device_id = sys_info.device_id().unwrap();
    println!("device_id: {}", device_id);

    let alias = sys_info.alias().unwrap();
    println!("alias: {}", alias);

    let model = sys_info.model().unwrap();
    println!("model: {}", model);

    let device_type = sys_info.device_type().unwrap();
    println!("device_type: {}", device_type);

    let mac_address = sys_info.mac_address().unwrap();
    println!("mac_address: {}", mac_address);
}
