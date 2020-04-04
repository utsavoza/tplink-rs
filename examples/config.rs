//! `cargo run --example config

use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = tplink::Config::for_host([192, 168, 1, 107])
        .with_read_timeout(Duration::from_secs(5))
        .with_cache_enabled(Duration::from_secs(3), None)
        .build();

    let mut bulb = tplink::Bulb::with_config(config);
    println!("{}", bulb.is_on()?);

    Ok(())
}
