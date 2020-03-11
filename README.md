tplink-rs
=========

[![CI](https://github.com/utsavoza/tplink-rs/workflows/CI/badge.svg)](https://github.com/utsavoza/tplink-rs/actions?query=workflow%3ACI)

A simple library to control TP-Link smart home devices.

## Examples
<!--
Add tplink-rs to your dependencies:
```toml
[dependencies]
tplink-rs = "0.1"
```
And then in your `main.rs`
-->
### Example - Discover

Discover existing TP-Link devices on your network.

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    for (ip, device) in tplink::discover()? {
        match device {
            tplink::DeviceKind::Plug(mut plug) => {
                println!("[{}] => {}", ip, plug.alias()?);
                
                plug.turn_on()?;
                assert_eq!(plug.is_on()?, true);
            },
            tplink::DeviceKind::Bulb(mut bulb) => {
                println!("[{}] => {}", ip, bulb.alias()?);

                bulb.turn_on()?;
                assert_eq!(bulb.is_on()?, true);

                if bulb.is_dimmable()? {
                    bulb.set_brightness(0)?;
                }
            },
            _ => println!("unrecognised device found with local address: {}", ip),
        }
    }   

    Ok(())
}
```

### Example - Bulb
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bulb = tplink::Bulb::new([192, 168, 1, 100]);
    
    bulb.turn_on()?;
    assert_eq!(bulb.is_on()?, true);

    if let Err(e) = bulb.set_brightness(60) {
        println!("{}", e);
    }

    bulb.turn_off()?;
    assert_eq!(bulb.is_on()?, false);

    Ok(())
}
```

### Example - Plug
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut plug = tplink::Plug::new([192, 168, 1, 100]);
    println!("alias: {}", plug.alias()?);
    println!("location: {}", plug.location()?);

    plug.turn_on()?;
    assert_eq!(plug.is_on()?, true);

    plug.turn_on_led()?;
    assert_eq!(plug.is_led_on()?, true);

    Ok(())
}
```

More examples can be found [here](examples).

## Currently Supported Devices

| Device  | Model         |
|---------|---------------|
| Plug    | HS100         |
| Bulb    | LB100, LB110  |


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
