# RusPiRo - Mailbox Property Tag Interface

This crate implements an abstraction of the mailbox property tag interface available in the Raspberry Pi.

Check the [official documentation](https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface) of those property tags and their purpose.

![CI](https://github.com/RusPiRo/ruspiro-mailbox/workflows/CI/badge.svg?branch=development)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-mailbox.svg)](https://crates.io/crates/ruspiro-mailbox)
[![Documentation](https://docs.rs/ruspiro-mailbox/badge.svg)](https://docs.rs/ruspiro-mailbox)
[![License](https://img.shields.io/crates/l/ruspiro-mailbox.svg)](https://github.com/RusPiRo/ruspiro-mailbox#license)

## Usage

To use the crate just add the following dependency to your ``Cargo.toml`` file:

```toml
[dependencies]
ruspiro-mailbox = "||VERSION||"
```

Once done the access to the mailbox interface access is available in your rust files like so:

```rust
use ruspiro_mailbox::*;

fn demo() {
    let mut mb = Mailbox::new();
    // use the mailbox to retrieve the core clock rate
    if let Ok(core_rate) = mb.get_clockrate(ArmClockId::Core) {
        // here we know the core clock rate do something with it...
        println!("Core clock rate {}", core_rate);
    }
}
```

## License

Licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0) or MIT ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)) at your choice.
