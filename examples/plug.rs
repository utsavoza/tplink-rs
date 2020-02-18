//! `cargo run --example plug`

use tplink::Result;

fn main() -> Result<()> {
    let plug = tplink::Plug::new([192, 168, 1, 100]);

    let sys_info = plug.sys_info()?;
    println!("sys_info: {}\n", sys_info);

    let res = plug.turn_on()?;
    println!("res: {}\n", res);

    let sys_info = plug.sys_info()?;
    println!("sys_info: {}\n", sys_info);

    let res = plug.turn_off()?;
    println!("res: {}\n", res);

    let sys_info = plug.sys_info()?;
    println!("sys_info: {}\n", sys_info);

    Ok(())
}
