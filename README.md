# Mailbox Property Tag Interface RusPiRo crate

This crate implements an abstraction of the mailbox property tag interface available in the Raspberry Pi.
There are currently a limmited number of functions for the following property tag messages implemented:
- GetArmMemory
- GetClockRate
- SetClockRate

Check the [official documentation](https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface) of those property tags and their purpose.
## Usage
To use the crate just add the following dependency to your ``Cargo.toml`` file:
```
[dependencies]
ruspiro-mailbox = "0.0.2"
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
This crate is licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)