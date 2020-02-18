tplink-rs
=========

A simple client library to control TP-Link smart devices.

Basic Usage
-----
```rust
use tplink::{Plug, Result};

fn main() -> Result<()> {
    let plug = Plug::new([192, 168, 1, 100]);

    let res = plug.turn_on()?;
    println!("res: {}\n", res);

    let sys_info = plug.sys_info()?;
    println!("sys_info: {}\n", sys_info);

    let res = plug.turn_off()?;
    println!("res: {}\n", res);

    Ok(())
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
