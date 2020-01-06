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

extern crate alloc;
extern crate paste;
use crate::{ClockId, DeviceId};

#[macro_use]
mod macros;

/// Proprty tag ID's used for the different mailbox messages
/// The same id's have to be used to define the property tag structures
/// as the mailbox request message tag id will be automatically set
#[repr(u32)]
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PropertyTagId {
    /// Retrieve ARM memory base address and size
    ArmMemoryGet = 0x1_0005,
    /// Retrieve the MAC address
    BoardMACAddressGet = 0x1_0003,
    /// Retrieve VC/GPU memory base address and size
    VcMemoryGet = 0x1_0006,
    /// Get the power state of a specific device
    PowerStateGet = 0x2_0001,
    /// Set the power state of a specific device
    PowerStateSet = 0x2_8001,
    /// Get the state of a specific clock
    ClockStateGet = 0x3_0001,
    /// Set the state of a specific clock
    ClockStateSet = 0x3_8001,
    /// Reading the current clock rate of a given clock ID
    ClockrateGet = 0x3_0002,
    /// Setting the current clockrate of a given clock ID
    ClockrateSet = 0x3_8002,
    /// Get the max possible rate for a given clock ID
    MaxClockrateGet = 0x3_0004,
    /* not yet implemented property tags
    FirmwareRevisionGet = 0x0_0001,
    BoardModelGet = 0x1_0001,
    BoardRevisionGet = 0x1_0002,
    BoardSerialGet = 0x1_0004,
    ClocksGet = 0x1_0007,
    TimingGet = 0x2_0002,

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
    */
}

/// The trait that each PropertyTag need to implement. The most convinient way to define mailbox
/// property tags is to use the corresponding macro. This will create the required
/// structure and implement this trait accoridingly
pub trait PropertyTag: Send {
    /// Type of the property tag request
    type Request;
    /// Type of the property tag response
    type Response;
    /// Return the [PropertyTagId] of this property tag
    fn tagid(&self) -> PropertyTagId;
    /// Return the current state of the property tag. This value is 0x0 for Requests and for a
    /// response the bit 31 is set to 1 and bits \[30..0\] contains the size of the response as it
    /// is known to the sender. If this value is greater then the payload passed as the size value
    /// for this tag, the response is truncated to fit into the response payload buffer provided.
    fn state(&self) -> u32;
    /// Return the reference to the response data of this property tag
    fn response(&self) -> &Self::Response;
    /// Returns the size of the property tag as defined by it's structure definition
    fn size(&self) -> u32;
}

property_tag!(
    /// Retrieve the current state of a clock
    ClockStateGet: {
        REQUEST: {
            clock_id: ClockId
        },
        RESPONSE: {
            clock_id: ClockId,
            state: u32
        }
    }
);

property_tag!(
    /// Set the state of a clock
    ClockStateSet: {
        REQUEST: {
            clock_id: ClockId,
            state: u32
        },
        RESPONSE: {
            clock_id: ClockId,
            state: u32
        }
    }
);

property_tag! (
    /// Retrieve the current clock rate in Hz. A value of 0 indicates that the
    /// specified clock does not exist. The rate is returned even if the clock is not active
    ClockrateGet: {
        REQUEST: {
            clock_id: ClockId
        },
        RESPONSE: {
            clock_id: ClockId,
            clock_rate: u32
        }
    }
);

property_tag!(
    /// Retrieve the maximum available rate in Hz for a clock
    MaxClockrateGet: {
        REQUEST: {
            clock_id: ClockId
        },
        RESPONSE: {
            clock_id: ClockId,
            clock_rate: u32
        }
    }
);

property_tag! (
    /// Set the current clock rate in Hz to the next supported value. Setting the Arm clock will
    /// activate turbo settings for other devices. This can be ommited by passing 'skip_turbo' as 1
    ClockrateSet: {
        REQUEST: {
            clock_id: ClockId,
            clock_rate: u32,
            skip_turbo: u32
        },
        RESPONSE: {
            clock_id: ClockId,
            clock_rate: u32
        }
    }
);

property_tag!(
    /// Get the memory base address and size that is dedicated to the ARM CPU. The split between
    /// Arm and GPU can be configured in the config.txt file that need to be present on the SD card
    /// while booting the Raspberry Pi.
    ArmMemoryGet: {
        REQUEST: {},
        RESPONSE: {
            base_address: u32,
            size: u32
        }
    }
);

property_tag!(
    /// Get the memory base and size that is dedicated to the VideoCore/GPU. The split between
    /// Arm and GPU can be configured in the config.txt file that need to be present on the SD card
    /// while booting the Raspberry Pi.
    VcMemoryGet: {
        REQUEST: {},
        RESPONSE: {
            base_address: u32,
            size: u32
        }
    }
);

property_tag!(
    /// Retrieve the power state of a device
    PowerStateGet: {
        REQUEST: {
            device_id: DeviceId
        },
        RESPONSE: {
            device_id: DeviceId,
            state: u32
        }
    }
);

property_tag!(
    /// Set the power sate of a device
    PowerStateSet: {
        REQUEST: {
            device_id: DeviceId,
            state: u32
        },
        RESPONSE: {
            device_id: DeviceId,
            state: u32
        }
    }
);

property_tag!(
    /// Retrieve the MAC address of this Raspberry Pi. The address is given in "network byte order"
    BoardMACAddressGet: {
        REQUEST: {
        },
        RESPONSE: {
            mac_address: [u8;6]
        },
        PADDING: u16
    }
);
