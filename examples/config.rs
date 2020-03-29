//! `cargo run --example config

use std::time::Duration;
use tplink::config::{self, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::for_host([192, 168, 1, 107]).build();
    let mut bulb = tplink::Bulb::with_config(config);
    println!("{}", bulb.is_on()?);

    let config = config::Builder::new([192, 168, 1, 101])
        .with_cache_enabled(Duration::from_secs(3), None)
        .build();
    let mut plug = tplink::Plug::with_config(config);
    println!("{}", plug.is_on()?);

    Ok(())
}
