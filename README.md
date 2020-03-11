tplink-rs
=========

A simple client library to control TP-Link smart home devices.

## Currently Supported Devices

| Device  | Model         |
|---------|---------------|
| Plug    | HS100         |
| Bulb    | LB100, LB110  |

## Example
Add tplink-rs to your dependencies:
```toml
[dependencies]
tplink-rs = "0.1"
```
And then in your `main.rs`
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bulb = tplink::Bulb::new([192, 168, 1, 100]);
    
    bulb.turn_on()?;
    assert_eq!(bulb.is_on()?, true);
    
    bulb.turn_off()?;
    assert_eq!(bulb.is_on()?, false);

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
