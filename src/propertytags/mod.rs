/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/

//! # Property Tag definitions
//!
//! This module defines all possible property tags and provides the message definitions for a set of property tags
//! that are currently supported and could be accessed with the corresponding functions of the Mailbox structure.
//!

extern crate paste;

use crate::{MailboxMessage, MessageState};

#[macro_use]
mod macros;

/// Proprty tag ID's used for the different mailbox messages
/// The same id's have to be used to define the property tag structures
/// as the mailbox request message tag id will be automatically set
#[repr(u32)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum PropertyTag {
    FirmwareRevisionGet = 0x0_0001,
    BoardModelGet = 0x1_0001,
    BoardRevisionGet = 0x1_0002,
    BoardMACAdressGet = 0x1_0003,
    BoardSerialGet = 0x1_0004,
    /// Retrieve ARM memory base address and size
    ArmMemoryGet = 0x1_0005,
    VcMemoryGet = 0x1_0006,
    ClocksGet = 0x1_0007,
    PowerStateGet = 0x2_0001,
    TimingGet = 0x2_0002,
    PowerStateSet = 0x2_8001,
    ClockStateGet = 0x3_0001,
    ClockStateSet = 0x3_8001,
    /// Reading the current clock rate of a given clock ID
    ClockrateGet = 0x3_0002,
    /// Setting the current clockrate of a given clock ID
    ClockrateSet = 0x3_8002,
    MaxClockrateGet = 0x3_0004,
    MinClockrateGet = 0x3_0007,
    TurboGet = 0x3_0009,
    TurboSet = 0x3_8009,
    VoltageGet = 0x3_0003,
    VoltageSet = 0x3_8003,
    MaxVoltageGet = 0x3_0005,
    MinVoltageGet = 0x3_0008,
    TemperatureGet = 0x3_0006,
    MaxTemperatureGet = 0x3_000A,
    MemoryAllocate = 0x3_000C,
    MemoryLock = 0x3_000D,
    MemoryUnlock = 0x3_000E,
    MemoryRelease = 0x3_000F,
    ExecuteCode = 0x3_0010,
    BufferAllocate = 0x4_0001,
    BufferRelease = 0x4_8001,
    BlankScreen = 0x4_0002,
    PhysicalSizeGet = 0x4_0003,
    PhysicalSizeSet = 0x4_8003,
    VirtualSizeGet = 0x4_0004,
    VirtualSizeSet = 0x4_8004,
    DepthGet = 0x4_0005,
    DepthSet = 0x4_8005,
    PixelOrderGet = 0x4_0006,
    PixelOrderSet = 0x4_8006,
    AlphaModeGet = 0x4_0007,
    AlphaModeSet = 0x4_8007,
    PitchGet = 0x4_0008,
    VirtualOffsetGet = 0x4_0009,
    VirtualOffsetSet = 0x4_8009,
    OverscanGet = 0x4_000A,
    OverscanSet = 0x4_800A,
    PaletteGet = 0x4_000B,
    PaletteSet = 0x4_800B,
}

property_tag_message! (
    ClockrateGet: {
        REQUEST: {
            clock_id: u32,
            clock_rate: u32
        },
        RESPONSE: {
            clock_id: u32,
            clock_rate: u32
        }
    }
);

property_tag_message! (
    ClockrateSet: {
        REQUEST: {
            clock_id: u32,
            clock_rate: u32,
            skip_turbo: u32
        },
        RESPONSE: {
            clock_id: u32,
            clock_rate: u32
        }
    }
);

property_tag_message!(
    ArmMemoryGet: {
        REQUEST: {},
        RESPONSE: {
            base_address: u32,
            size: u32
        }
    }
);
