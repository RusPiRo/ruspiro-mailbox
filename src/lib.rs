/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 * 
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-mailbox/||VERSION||")]
#![cfg_attr(not(any(test, doctest)), no_std)]

//! # Mailbox property tag interface API
//!
//! This crate provides an abstraction to the mailbox property tag interface available on the Raspberry Pi.
//! There are currently the following property tag messages implemented:
//!
//! - FirmwareRevisionGet
//! - BoardModelGet
//! - BoardRevisionGet
//! - BoardSerialGet
//! - ArmMemoryGet
//! - BoardMACAddressGet
//! - VcMemoryGet
//! - DmaChannelsGet
//! - PowerStateGet
//! - PowerStateSet
//! - ClockStateGet
//! - ClockStateSet
//! - ClockrateGet
//! - ClockrateSet
//! - MaxClockrateGet
//! - MinClockrateGet
//! - VoltageGet
//! - VoltageSet
//! - MaxVoltageGet
//! - MinVoltageGet
//! - TemperatureGet
//! - MaxTemperatureGet
//! - FramebufferAllocate
//! - FramebufferRelease
//! - BlankScreen
//! - PhysicalSizeGet
//! - PhysicalSizeSet
//! - VirtualSizeGet
//! - VirtualSizeSet
//! - DepthGet
//! - DepthSet
//! - PixelOrderGet
//! - PixelOrderSet
//! - AlphaModeGet
//! - AlphaModeSet
//! - PitchGet
//! - VirtualOffsetGet
//! - VirtualOffsetSet
//! - OverscanGet
//! - OverscanSet
//! - PaletteGet
//! - PaletteSet
//!
//! Check the [official documentation](https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface)
//! of those property tags and their purpose.
//!
//! # Usage
//!
//! The crate provides a singleton wrapper to call the different Raspberry Pi mailbox property tag messages. The
//! following example demonstrates the usage with the GetClockRate message.
//!
//! ```no_run
//! use ruspiro_mailbox::*;
//!
//! fn main() {
//!     let mut mb = Mailbox::new();
//!     // use the mailbox to retrieve the core clock rate
//!     if let Ok(core_rate) = mb.get_clockrate(ClockId::Core) {
//!         // here we know the core clock rate, so do something with it...
//!         println!("Core clock rate {}", core_rate);
//!     }
//! }
//! ```
//!
//! It is also possible to create [MailboxBatch] messages that hold a list of [PropertyTag]s that
//! shall be processed by the GPU. This is for example the required format if it comes to the framebuffer
//! setup. Each [MailboxBatch] can contain any [PropertyTag] only once. After the [MailboxBatch] has
//! been processed the individual response of a specific [PropertyTag] can be inspected and checked for
//! the desired value. The success status on batch level only indicates whether all tags have been
//! processed successfull or not. If only parts could be successfully processed each tag need to be
//! verified which one has failed.
//!
//! # Example
//!
//! ```no_run
//! # use ruspiro_mailbox::*;
//!
//! fn main() {
//!     let mut mb = Mailbox::new();
//!     // first create a new empty batch
//!     let mut batch = MailboxBatch::empty()
//!     // add as many tags as required to the batch
//!         .with_tag(ClockrateGet::new(ClockId::Core))
//!         .with_tag(MaxClockrateGet::new(ClockId::Arm));
//!
//!     // execute the batch using the mailbox peripheral
//!     if let Ok(batch) = mb.send_batch(batch) {
//!         // as the batch processing has been successfull we can check individual
//!         // tag responses
//!         println!("Core clock rate: {}",
//!             batch.get_tag::<ClockrateGet, _>().response().clock_rate());
//!         println!("Max Arm clock rate: {}",
//!             batch.get_tag::<MaxClockrateGet, _>().response().clock_rate());
//!     }
//! }
//! ```
//!
//! # Hint
//!
//! The `Mailbox` represents a kind of a peripheral device of the Raspberry Pi. Therefore it should only be instantiated
//! once. As the `Mailbox` is not capable to handle multiple parallel requests the `Mailbox` methods always require a 
//! mutable reference of it when called. This ensures exclusive access to the mailbox interface for message calls.
//!
//! # Features
//!
//! - `ruspiro_pi3` When active it ensures the proper MMIO base address is compiled for Raspberry Pi 3
//!

use ruspiro_error::BoxError;

mod interface;
use interface::*;
mod propertytags;
pub use propertytags::*;
mod message;
pub use message::*;

/// Definition of the different clock id's used in the mailbox interface
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ClockId {
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
#[derive(Copy, Clone, Debug)]
pub enum DeviceId {
    SdCard = 0x0000_0000,
    Uart0 = 0x0000_0001,
    Uart1 = 0x0000_0002,
    UsbHcd = 0x0000_0003,
    I2C0 = 0x0000_0004,
    I2C1 = 0x0000_0005,
    I2C2 = 0x0000_0006,
    Spi = 0x0000_0007,
    Ccp2Tx = 0x0000_0008,
}

/// Definition of the different Voltage Id's on Raspberry Pi
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum VoltageId {
    Core = 0x01,
    SdRamC = 0x02,
    SdRamP = 0x03,
    SdRamI = 0x04,
}

/// Definition of the different mailbox channels to be used for communication
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum MailboxChannel {
    /// Power management channel
    PowerMgmt = 0x0,
    /// Framebuffer channel (shall not be used)
    FrameBuffer = 0x1,
    /// Virtual UART channel
    VirtualUart = 0x2,
    /// Property tag channel to send from ARM to VideoCore
    PropertyTagsVc = 0x8,
    /// Property tag channel to send from VideoCore to ARM
    PropertyTagsArm = 0x9,
}

/// Definition of the different message stats/types used in the mailbox interface
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum MessageState {
    /// All outgoing messages are of the request type
    Request = 0x0,
    /// If the message has been successfull processed by the receiver
    ResponseOk = 0x8000_0000,
    /// If the message hs not been successfully or just partly successfully processed by the receiver
    ResponseError = 0x8000_0001,
}

/// Type alias for Results of the functions in this module
pub type MailboxResult<T> = Result<T, BoxError>;

/// MAILBOX peripheral representation
pub struct Mailbox;

impl Mailbox {
    pub const fn new() -> Self {
        Mailbox
    }

    /// Send a mailbox batch message
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let batch = MailboxBatch::empty()
    ///                 .with_tag(ClockrateGet::new(ClockId::Core));
    /// let _ = mb.send_batch(batch);
    /// # }
    /// ```
    pub fn send_batch<T>(&mut self, batch: MailboxBatch<T>) -> MailboxResult<MailboxBatch<T>> {
        send_batch(MailboxChannel::PropertyTagsVc, batch)
    }

    /// Get the firmware revision of this Raspberry Pi
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let revision = mb.get_firmware_revision().unwrap();
    /// # }
    /// ```
    pub fn get_firmware_revision(&mut self) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            FirmwareRevisionGet::new().into(),
        )
        .map(|message| message.response().firmware_revision())
    }

    /// Get the board model of this Raspberry Pi
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let model = mb.get_board_model().unwrap();
    /// # }
    /// ```
    pub fn get_board_model(&mut self) -> MailboxResult<u32> {
        send_message(MailboxChannel::PropertyTagsVc, BoardModelGet::new().into())
            .map(|message| message.response().board_model())
    }

    /// Get the board revision of this Raspberry Pi.
    /// Check out https://www.raspberrypi.org/documentation/hardware/raspberrypi/revision-codes/README.md
    /// for the encoding of the returned value
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let revision = mb.get_board_revision().unwrap();
    /// # }
    /// ```
    pub fn get_board_revision(&mut self) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            BoardRevisionGet::new().into(),
        )
        .map(|message| message.response().board_revision())
    }

    /// Get the MAC address of this Raspberry Pi
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let mac_addr = mb.get_board_mac_address().unwrap();
    /// # }
    /// ```
    pub fn get_board_mac_address(&mut self) -> MailboxResult<[u8; 6]> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            BoardMACAddressGet::new().into(),
        )
        .map(|message| message.response().mac_address())
    }

    /// Get the ARM memory base address and size as configured in the boot config file.
    /// Returns a tuple Ok((address:u32, size:u32)) on success or an Err(msg: &str) on failure
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let (address, size) = mb.get_arm_memory().unwrap();
    /// # }
    /// ```
    pub fn get_arm_memory(&mut self) -> MailboxResult<(u32, u32)> {
        send_message(MailboxChannel::PropertyTagsVc, ArmMemoryGet::new().into()).map(|message| {
            let response = message.response();
            (response.base_address(), response.size())
        })
    }

    /// Get the VideoCore memory base address and size as configured in the boot config file.
    /// Returns a tuple Ok((address:u32, size:u32)) on success or an Err(msg: &str) on failure
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let (address, size) = mb.get_vc_memory().unwrap();
    /// # }
    /// ```
    pub fn get_vc_memory(&mut self) -> MailboxResult<(u32, u32)> {
        send_message(MailboxChannel::PropertyTagsVc, VcMemoryGet::new().into()).map(|message| {
            let response = message.response();
            (response.base_address(), response.size())
        })
    }

    /// Get the active DMA channels.<br>
    /// Bits 0-15  of the response represents the DMA channels 0-15. If the corresponding bit is set for a
    /// channel it is usable. Bits 16-31 are reserved
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let dma_channel = mb.get_dma_channels().unwrap();
    /// # }
    /// ```
    pub fn get_dma_channels(&mut self) -> MailboxResult<u32> {
        send_message(MailboxChannel::PropertyTagsVc, DmaChannelsGet::new().into())
            .map(|message| message.response().channel_mask())
    }

    /// Get the power state of the given device id.
    /// The returned state could have the following values:
    /// Bit 0: 0 = off, 1 = on
    /// Bit 1: 0 = device exists, 1 = device unknown
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let state = mb.get_powerstate(DeviceId::Uart0).unwrap();
    /// # }
    /// ```
    pub fn get_powerstate(&mut self, device_id: DeviceId) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            PowerStateGet::new(device_id).into(),
        )
        .map(|message| message.response().state())
    }

    /// Set the power state of the given device id.
    /// The state to be set should contain those values:
    /// Bit 0: 0 = off, 1 = on
    /// Bit 1: 0 = don't wait for device state change, 1 = wait for device state change
    /// The returned state contains the following values:
    /// Bit 0: 0 = off, 1 = on
    /// Bit 1: 0 = device exists, 1 = device unknown
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let new_state = mb.set_powerstate(DeviceId::Uart0, 0b11).unwrap();
    /// # }
    /// ```
    pub fn set_powerstate(&mut self, device_id: DeviceId, state: u32) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            PowerStateSet::new(device_id, state).into(),
        )
        .map(|message| message.response().state())
    }

    /// Get the clock state of the given clock id.
    /// The returned state could have the following values:
    /// Bit 0: 0 = off, 1 = on
    /// Bit 1: 0 = clock exists, 1 = clock unknown
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let state = mb.get_clockstate(ClockId::Core).unwrap();
    /// # }
    /// ```
    pub fn get_clockstate(&mut self, clock_id: ClockId) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            ClockStateGet::new(clock_id).into(),
        )
        .map(|message| message.response().state())
    }

    /// Set the clock state of the given clock id.
    /// The state to be set should contain those values:
    /// Bit 0: 0 = off, 1 = on
    /// The returned state contains the following values:
    /// Bit 0: 0 = off, 1 = on
    /// Bit 1: 0 = clock exists, 1 = clock unknown
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let new_state = mb.set_clockstate(ClockId::Core, 0b1).unwrap();
    /// # }
    /// ```
    pub fn set_clockstate(&mut self, clock_id: ClockId, state: u32) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            ClockStateSet::new(clock_id, state).into(),
        )
        .map(|message| message.response().state())
    }

    /// Get the clock rate via mailbox interface for the clockId given.
    /// Returns Ok(rate:u32) on success or Err(msg: &str) on failure
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let clock_rate = mb.get_clockrate(ClockId::Core).unwrap();
    /// # }
    /// ```
    pub fn get_clockrate(&mut self, clock_id: ClockId) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            ClockrateGet::new(clock_id).into(),
        )
        .map(|message| message.response().clock_rate())
    }

    /// Set the clock rate via the mailbox interface for the clockId given. The rate will be set to the closest valid
    /// value. Returns Ok(rate:u32) with the new clock rate set on success ore Err(msg: &str) on failure
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let new_clock_rate = mb.set_clockrate(ClockId::Core, 250_000_000).unwrap();
    /// # }
    /// ```
    pub fn set_clockrate(&mut self, clock_id: ClockId, rate: u32) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            ClockrateSet::new(clock_id, rate, 0x0).into(),
        )
        .map(|message| message.response().clock_rate())
    }

    /// Get the maximum available clock rate for the given clock id
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let max_rate = mb.get_max_clockrate(ClockId::Core).unwrap();
    /// # }
    /// ```
    pub fn get_max_clockrate(&mut self, clock_id: ClockId) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            MaxClockrateGet::new(clock_id).into(),
        )
        .map(|message| message.response().clock_rate())
    }

    /// Get the minimum available clock rate for the given clock id
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let min_rate = mb.get_min_clockrate(ClockId::Core).unwrap();
    /// # }
    /// ```
    pub fn get_min_clockrate(&mut self, clock_id: ClockId) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            MinClockrateGet::new(clock_id).into(),
        )
        .map(|message| message.response().clock_rate())
    }

    /// Get the current voltage of the given [VoltageId]. The value represents an offset from
    /// 1.2V in units of 0.025V.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let voltage = mb.get_voltage(VoltageId::Core).unwrap();
    /// # }
    /// ```
    pub fn get_voltage(&mut self, voltage_id: VoltageId) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            VoltageGet::new(voltage_id).into(),
        )
        .map(|message| message.response().value())
    }

    /// Set the current voltage for the given [VoltageId]. The value represents an offset from
    /// 1.2V in units of 0.025V.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let new_voltage = mb.set_voltage(VoltageId::Core, 10).unwrap();
    /// # }
    /// ```
    pub fn set_voltage(&mut self, voltage_id: VoltageId, value: u32) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            VoltageSet::new(voltage_id, value).into(),
        )
        .map(|message| message.response().value())
    }

    /// Get the maximum voltage of the given [VoltageId]. The value represents an offset from
    /// 1.2V in units of 0.025V.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let max_voltage = mb.get_max_voltage(VoltageId::Core).unwrap();
    /// # }
    /// ```
    pub fn get_max_voltage(&mut self, voltage_id: VoltageId) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            MaxVoltageGet::new(voltage_id).into(),
        )
        .map(|message| message.response().value())
    }

    /// Get the minimum voltage of the given [VoltageId]. The value represents an offset from
    /// 1.2V in units of 0.025V.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let min_voltage = mb.get_min_voltage(VoltageId::Core).unwrap();
    /// # }
    /// ```
    pub fn get_min_voltage(&mut self, voltage_id: VoltageId) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            MinVoltageGet::new(voltage_id).into(),
        )
        .map(|message| message.response().value())
    }

    /// Get the current temperature in thousandths of a degree Celsius.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let temperature = mb.get_temperature().unwrap();
    /// # }
    /// ```
    pub fn get_temperature(&mut self) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            TemperatureGet::new(0x0).into(),
        )
        .map(|message| message.response().value())
    }

    /// Get the maximum safe temperature in thousandths of a degree Celsius. Above this temperature
    /// overclocking/turbo might get deactivated
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let max_temp = mb.get_max_temperature().unwrap();
    /// # }
    /// ```
    pub fn get_max_temperature(&self) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            MaxTemperatureGet::new(0x0).into(),
        )
        .map(|message| message.response().value())
    }

    /// Send the VCHIQ slots base address to the VideoCore
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruspiro_mailbox::*;
    /// # fn doc() {
    /// let mut mb = Mailbox::new();
    /// let _ = mb.set_vchiq_slot_base(0xBEEF_0000).unwrap();
    /// # }
    /// ```
    pub fn set_vchiq_slot_base(&mut self, slot_base: u32) -> MailboxResult<u32> {
        send_message(
            MailboxChannel::PropertyTagsVc,
            VchiqInit::new(slot_base).into(),
        )
        .map(|message| message.response().status())
    }
}
