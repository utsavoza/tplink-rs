//! `cargo run --example plug`

fn main() {
    let plug = tplink::Plug::new([192, 168, 1, 100]);

    let sys_info = plug.get_sys_info().unwrap();
    println!("sys_info: {}\n", sys_info);

    //    let res = plug.switch_on().unwrap();
    //    println!("res: {}\n", res);
    //
    //    let sys_info = plug.get_sys_info().unwrap();
    //    println!("sys_info: {}\n", sys_info);
    //
    //    let res = plug.switch_off().unwrap();
    //    println!("res: {}\n", res);
    //
    //    let sys_info = plug.get_sys_info().unwrap();
    //    println!("sys_info: {}\n", sys_info);
}
