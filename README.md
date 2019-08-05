# RusPiRo - Mailbox Property Tag Interface

This crate implements an abstraction of the mailbox property tag interface available in the Raspberry Pi.
There are currently a limmited number of functions for the following property tag messages implemented:
- GetArmMemory
- GetClockRate
- SetClockRate

Check the [official documentation](https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface) of those property tags and their purpose.

[![Travis-CI Status](https://api.travis-ci.org/RusPiRo/ruspiro-mailbox.svg?branch=master)](https://travis-ci.org/RusPiRo/ruspiro-mailbox)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-mailbox.svg)](https://crates.io/crates/ruspiro-mailbox)
[![Documentation](https://docs.rs/ruspiro-mailbox/badge.svg)](https://docs.rs/ruspiro-mailbox)
[![License](https://img.shields.io/crates/l/ruspiro-mailbox.svg)](https://github.com/RusPiRo/ruspiro-mailbox#license)


## Usage

To use the crate just add the following dependency to your ``Cargo.toml`` file:
```
[dependencies]
ruspiro-mailbox = "0.1.0"
```

Once done the access to the mailbox interface access is available in your rust files like so:
```
use ruspiro_mailbox::*;

fn demo() {
    // use the mailbox to retrieve the core clock rate
    if let Ok(core_rate) = MAILBOX.take_for(|mb| mb.get_clockrate(ArmClockId::Core)) {
        // here we know the core clock rate do something with it...
        // remeber - println is just a show case and might not be available in bare metal environment
        println!("Core clock rate {}", core_rate);
    }
}
```

## License
Licensed under Apache License, Version 2.0, ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)