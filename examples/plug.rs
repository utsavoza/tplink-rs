//! `cargo run --example plug`

fn main() {
    env_logger::init();

    let mut plug = tplink::Plug::new([192, 168, 1, 100]);

    let sysinfo = plug.sysinfo().unwrap();
    println!("sys_info: {:?}", sysinfo);

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
