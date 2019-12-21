/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-mailbox/0.3.0")]
#![no_std]
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
//! ```no_run
//! use ruspiro_mailbox::*;
//!
//! fn main() {
//!     // use the mailbox to retrieve the core clock rate
//!     if let Ok(core_rate) = MAILBOX.take_for(|mb| mb.get_clockrate(ArmClockId::Core)) {
//!         // here we know the core clock rate do something with it...
//!         println!("Core clock rate {}", core_rate);
//!     }
//! }
//! ```
//! 
//! # Features
//! - ``ruspiro_pi3`` is active by default and ensures the proper MMIO base address is compiled for Raspberry Pi 3
//! 

use ruspiro_singleton::Singleton;

mod interface;
use interface::*;
mod propertytags;
use propertytags::*;

/// static "singleton" accessor to the MAILBOX peripheral
pub static MAILBOX: Singleton<Mailbox> = Singleton::new(Mailbox::new());

/// Definition of the different ARM clock id's used in the mailbox interface
#[repr(u32)]
pub enum ArmClockId {
    Emmc = 0x1,
    Uart = 0x2,
    Arm = 0x3,
    Core = 0x4,
    V3D = 0x5,
    H264 = 0x6,
    Isp = 0x7,
    SdRam = 0x8,
    Pixel = 0x9,
    Pwm = 0xa,
    Emmc2 = 0xc,
}

/// Definition of the different Unique Device Id's on Raspberry Pi
#[repr(u32)]
pub enum DeviceId {
    SdCard = 0x00000000,
    Uart0 = 0x00000001,
    Uart1 = 0x00000002,
    UsbHcd = 0x00000003,
    I2C0 = 0x00000004,
    I2C1 = 0x00000005,
    I2C2 = 0x00000006,
    Spi = 0x00000007,
    Ccp2Tx = 0x00000008,
}

/// MAILBOX peripheral representation
pub struct Mailbox;

impl Mailbox {
    pub(crate) const fn new() -> Self {
        Mailbox
    }

    /// Get the ARM memory base address and size as configured in the boot config file.
    /// Returns a tuple Ok((address:u32, size:u32)) on success or an Err(msg: &str) on failure
    /// # Example
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn demo() {
    /// let arm_memory = MAILBOX.take_for(|mb| mb.get_arm_memory()).unwrap();
    /// println!("ARM memory address: {}, size: {}", arm_memory.0, arm_memory.1);
    /// # }
    /// ```
    pub fn get_arm_memory(&self) -> MailboxResult<(u32, u32)> {
        send_message(MailboxChannel::PropertyTagsVc, ArmMemoryGet::new()).map(|message| {
            let response = message.get_response();
            (response.base_address, response.size)
        })
    }

    /// Get the VideoCore memory base address and size as configured in the boot config file.
    /// Returns a tuple Ok((address:u32, size:u32)) on success or an Err(msg: &str) on failure
    /// # Example
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn demo() {
    /// let vc_memory = MAILBOX.take_for(|mb| mb.get_vc_memory()).unwrap();
    /// println!("VC memory address: {}, size: {}", vc_memory.0, vc_memory.1);
    /// # }
    /// ```
    pub fn get_vc_memory(&self) -> MailboxResult<(u32, u32)> {
        send_message(MailboxChannel::PropertyTagsVc, VcMemoryGet::new()).map(|message| {
            let response = message.get_response();
            (response.base_address, response.size)
        })
    }

    /// Get the clock state of the given clock id.
    /// The returned state could have the following values:
    /// Bit 0: 0 = off, 1 = on
    /// Bit 1: 0 = clock exists, 1 = clock unknown
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn demo() {
    /// let clock_state = MAILBOX.take_for(|mb| mb.get_clockstate(ArmClockId::Pwm)).unwrap();
    /// # }
    /// ```
    pub fn get_clockstate(&self, clock_id: ArmClockId) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            ClockStateGet::new(clock_id as u32),
        )
        .map(|message| message.get_response().state)
    }

    /// Set the clock state of the given clock id.
    /// The state to be set should contain those values:
    /// Bit 0: 0 = off, 1 = on
    /// The returned state contains the following values:
    /// Bit 0: 0 = off, 1 = on
    /// Bit 1: 0 = clock exists, 1 = clock unknown
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn demo() {
    /// let new_clock_state = MAILBOX.take_for(|mb| mb.set_clockstate(ArmClockId::Pwm, 0b1))
    ///     .unwrap();
    /// # }
    /// ```
    pub fn set_clockstate(&self, clock_id: ArmClockId, state: u32) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            ClockStateSet::new(clock_id as u32, state),
        )
        .map(|message| message.get_response().state)
    }

    /// Get the clock rate via mailbox interface for the clockId given.
    /// Returns Ok(rate:u32) on success or Err(msg: &str) on failure
    /// # Example
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn demo() {
    /// let clock_rate = MAILBOX.take_for(|mb| mb.get_clockrate(ArmClockId::Core)).unwrap();
    /// # }
    /// ```
    pub fn get_clockrate(&self, clock_id: ArmClockId) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            ClockrateGet::new(clock_id as u32, 0x0),
        )
        .map(|message| message.get_response().clock_rate)
    }

    /// Set the clock rate via the mailbox interface for the clockId given. The rate will be set to the closest valid value.
    /// Returns Ok(rate:u32) with the new clock rate set on success ore Err(msg: &str) on failure
    /// # Example
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn demo() {
    /// let new_clock_rate = MAILBOX.take_for(|mb| mb.set_clockrate(ArmClockId::Core, 250_000_000))
    ///     .unwrap();
    /// # }
    /// ```
    pub fn set_clockrate(&self, clock_id: ArmClockId, rate: u32) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            ClockrateSet::new(clock_id as u32, rate, 0x0),
        )
        .map(|message| message.get_response().clock_rate)
    }

    /// Get the power state of the given device id.
    /// The returned state could have the following values:
    /// Bit 0: 0 = off, 1 = on
    /// Bit 1: 0 = device exists, 1 = device unknown
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn demo() {
    /// let power_state = MAILBOX.take_for(|mb| mb.get_powerstate(DeviceId::SdCard)).unwrap();
    /// # }
    /// ```
    pub fn get_powerstate(&self, device_id: DeviceId) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            PowerStateGet::new(device_id as u32),
        )
        .map(|message| message.get_response().state)
    }

    /// Set the power state of the given device id.
    /// The state to be set should contain those values:
    /// Bit 0: 0 = off, 1 = on
    /// Bit 1: 0 = don't wait for device state change, 1 = wait for device state change
    /// The returned state contains the following values:
    /// Bit 0: 0 = off, 1 = on
    /// Bit 1: 0 = device exists, 1 = device unknown
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn demo() {
    /// let new_power_state = MAILBOX.take_for(|mb| mb.set_powerstate(DeviceId::SdCard, 0b11))
    ///     .unwrap();
    /// # }
    /// ```
    pub fn set_powerstate(&self, device_id: DeviceId, state: u32) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            PowerStateSet::new(device_id as u32, state),
        )
        .map(|message| message.get_response().state)
    }
}
