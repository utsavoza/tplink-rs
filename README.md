tplink-rs
=========

[![CI](https://github.com/utsavoza/tplink-rs/workflows/CI/badge.svg)](https://github.com/utsavoza/tplink-rs/actions?query=workflow%3ACI)

A simple library to control TP-Link Smart Home devices.

## Examples
<!--
Add tplink-rs to your dependencies:
```toml
[dependencies]
tplink = "0.1"
```
And then in your `main.rs`
-->
### Discover

Discover existing TP-Link devices on your network.

```rust
use tplink::DeviceKind;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let devices = tplink::discover()?;

    for (ip, device) in devices {
        match device {
            DeviceKind::Plug(mut plug) => {
                if plug.is_on()? {
                    plug.turn_off()?;
                    assert_eq!(plug.is_on()?, false);
                }
            }
            DeviceKind::Bulb(mut bulb) => {
                if bulb.is_on()? && bulb.is_dimmable()? {
                    bulb.set_brightness(50)?;
                    assert_eq!(bulb.brightness()?, 50);
                }
            }
            _ => eprintln!("unrecognised device found on the network: {}", ip),
        }
    }

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
