/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Apache License 2.0
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-mailbox/0.0.2")]
#![no_std]
#![feature(asm)]

//! # Mailbox property tag interface
//! 
//! This crate provides an abstraction on the mailbox property tag interface available in the Raspberry Pi.
//! There are currently a limmited number of functions for the following property tag messages implemented:
//! - GetArmMemory
//! - GetClockRate
//! - SetClockRate
//! 
//! Check the [official documentation](https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface) of those property tags and their purpose.
//! 
//! # Usage
//! 
//! The crate provides a singleton wrapper to call the different Raspberry Pi mailbox property tag messages. The
//! following example demonstrates the usage with the GetClockRate message.
//! ```
//! use ruspiro_mailbox::*;
//! 
//! fn demo() {
//!     // use the mailbox to retrieve the core clock rate
//!     if let Ok(core_rate) = MAILBOX.take_for(|mb| mb.get_clockrate(ArmClockId::Core)) {
//!         // here we know the core clock rate do something with it...
//!         // remeber - println is just a show case here as it might not be available in bare metal environment
//!         println!("Core clock rate {}", core_rate);
//!     }
//! }
//! ```

use ruspiro_singleton::Singleton;

mod interface;
mod propertytags;
use interface::*;

// MMIO base address for peripherals
const PERIPHERAL_BASE: u32 = 0x3F00_0000;

// Mailbox MMIO base address
const MAILBOX_BASE: u32 = PERIPHERAL_BASE + 0x0000_B880;

/// static "singleton" accessor to the MAILBOX peripheral
pub static MAILBOX: Singleton<Mailbox> = Singleton::new(Mailbox::new());

/// Definition of the different ARM clock id's used in the mailbox interface
#[repr(u32)]
pub enum ArmClockId {
    Emmc   = 0x1,
    Uart   = 0x2,
    Arm    = 0x3,
    Core   = 0x4
}

/// MAILBOX peripheral representation
pub struct Mailbox;

impl Mailbox {
    pub(crate) const fn new() -> Self {
        Mailbox
    }

    /// Get the ARM memory base address and size as configured in the boot config file
    pub fn get_arm_memory(&self) -> MailboxResult<(u32, u32)> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            &propertytags::ArmMemoryGet::new()
        ).and_then(|tag| {
            let response = tag.get_response();
            Ok((response.base_address, response.size))
        })
    }

    /// Get the clock rate via mailbox interface for the clockId given
    pub fn get_clockrate(&self, clock_id: ArmClockId) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            &propertytags::ClockrateGet::new(clock_id as u32)
        ).and_then(|clock_rate_get| {
            Ok(clock_rate_get.get_response().clock_rate)
        })
    }

    /// Set the clock rate via the mailbox interface for the clockId given. The rate will be set to the closest valid value.
    pub fn set_clockrate(&self, clock_id: ArmClockId, rate: u32) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            &propertytags::ClockrateSet::new(clock_id as u32, rate, 0x0)
        ).and_then(|clock_rate_set| {
            Ok(clock_rate_set.get_response().clock_rate)
        })
    }
}

