tplink-rs
=========

A simple client library to control TP-Link smart devices.

Basic Usage
-----
```rust
fn main() {
    let plug = tplink::Plug::new([192, 168, 1, 100]);
    let sys_info = plug.sys_info().unwrap();
    println!("plug - sys_info: {}", sys_info);
    
    let bulb = tplink::Bulb::new([192, 168, 1, 101]);
    let sys_info = bulb.sys_info().unwrap();
    println!("bulb - sys_info: {}", sys_info);
}
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>