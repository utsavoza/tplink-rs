tplink-rs
=========

A simple client library to control TP-Link smart home devices.

## Example
Add tplink-rs to your dependencies:
```toml
[dependencies]
tplink-rs = "0.1"
```
And then in your `main.rs` or `lib.rs`
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