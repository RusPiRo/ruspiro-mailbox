/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/

//! # Property Tag definitions
//!
//! This module defines all possible property tags and provides the message definitions for a set of
//! property tags that are currently supported and could be accessed with the corresponding functions
//! of the Mailbox accessor or being used as part of a batch request.
//!
//! # Example
//!
//! ```no_run
//! # use ruspiro_mailbox::*;
//! # fn doc() {
//! let mut mb = Mailbox::new();
//! // create a property tag to request the clock rate of the core clock.
//! let tag = ClockrateGet::new(ClockId::Core);
//! // this could be used in a batch message like so
//! let batch = MailboxBatch::empty().with_tag(tag);
//! if let Ok(response) = mb.send_batch(batch) {
//!   println!("Core rate: {}", response.get_tag::<ClockrateGet, _>().response().clock_rate());
//! }
//!
//! // more convinient for single property tags to be processed is to use the corresponding
//! // functions of the mailbox accessor
//! if let Ok(clock_rate) = mb.get_clockrate(ClockId::Core) {
//!     println!("Core rate: {}", clock_rate);
//! }
//! # }
//! ```

use crate::{ClockId, DeviceId, VoltageId};

#[macro_use]
mod macros;

/// Proprty tag ID's used for the different mailbox messages
/// The same id's have to be used to define the property tag structures
/// as the mailbox request message tag id will be automatically set
#[repr(u32)]
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum PropertyTagId {
    /// Retrieve the firmware revision code
    FirmwareRevisionGet = 0x0_0001,
    /// Retrieve the board model code
    BoardModelGet = 0x1_0001,
    /// Retrieve the board revision code.
    /// Check https://www.raspberrypi.org/documentation/hardware/raspberrypi/revision-codes/README.md
    /// for a decoding of the returned value.
    BoardRevisionGet = 0x1_0002,
    /// Retrieve the serial number
    BoardSerialGet = 0x1_0004,
    /// Retrieve ARM memory base address and size
    ArmMemoryGet = 0x1_0005,
    /// Retrieve the MAC address
    BoardMACAddressGet = 0x1_0003,
    /// Retrieve VC/GPU memory base address and size
    VcMemoryGet = 0x1_0006,
    /// Retrieve all usable DMA channels
    DmaChannelsGet = 0x6_0001,
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
    /// Get the minimal possible rate for a given clock ID
    MinClockrateGet = 0x3_0007,
    /// Get the current voltage value for the given [VoltageId]
    VoltageGet = 0x3_0003,
    /// Set the current voltage value for the given [VoltageId]
    VoltageSet = 0x3_8003,
    /// Get the maximum voltage value for the given [VoltageId]
    MaxVoltageGet = 0x3_0005,
    /// Get the minimum voltage value for the given [VoltageId]
    MinVoltageGet = 0x3_0008,
    /// Retrieve the current temperature in thousandths of a degree Celsius
    TemperatureGet = 0x3_0006,
    /// Retrieve the maximum safe temperature in thousandths of a degree Celsius
    MaxTemperatureGet = 0x3_000A,
    /// Allocate a frame buffer based on the size and pixel config given in a batch mailbox message
    FramebufferAllocate = 0x4_0001,
    /// Release and disable the frame buffer
    FramebufferRelease = 0x4_8001,
    /// clear the framebuffer (only the virtual part?)
    BlankScreen = 0x4_0002,
    /// Retrieve the current physical (display) size of the frame buffer
    PhysicalSizeGet = 0x4_0003,
    /// Set the physical (display) size of the frame buffer (allocation will be made based on this size, even
    /// though there might be pitching applied to the requested width)
    PhysicalSizeSet = 0x4_8003,
    /// Retrieve the virtual display size
    VirtualSizeGet = 0x4_0004,
    /// Set the virtual display size that has to be less than or equal to the physical size and is the
    /// visible part of the frame buffer
    VirtualSizeSet = 0x4_8004,
    /// Get the actual pixel bit depth
    DepthGet = 0x4_0005,
    /// Set the actual pixel bit depth
    DepthSet = 0x4_8005,
    /// Retrieve the current pixel color ordering
    PixelOrderGet = 0x4_0006,
    /// Set the pixel color ordering (RGB or BGR)
    PixelOrderSet = 0x4_8006,
    /// Retrieve the current alpha mode setting
    AlphaModeGet = 0x4_0007,
    /// Set the alpha mode
    AlphaModeSet = 0x4_8007,
    /// Get the actual pitch for the physical (display) size
    PitchGet = 0x4_0008,
    /// Retrieve the current offset of the virtual buffer into the physical one
    VirtualOffsetGet = 0x4_0009,
    /// Set the offset of the virtual buffer into the physical one in pixels
    VirtualOffsetSet = 0x4_8009,
    /// Retrieve the current overscan settings
    OverscanGet = 0x4_000A,
    /// Set the overscan values
    OverscanSet = 0x4_800A,
    /// Retrieve the current used palette color values
    PaletteGet = 0x4_000B,
    /// Set/Update the palette color values
    PaletteSet = 0x4_800B,
    /// VideoCore Host Interface initialization
    VchiqInit = 0x4_8010,
    /* not yet implemented property tags
    ClocksGet = 0x1_0007,
    TimingGet = 0x2_0002,

    TurboGet = 0x3_0009,
    TurboSet = 0x3_8009,

    MemoryAllocate = 0x3_000C,
    MemoryLock = 0x3_000D,
    MemoryUnlock = 0x3_000E,
    MemoryRelease = 0x3_000F,
    ExecuteCode = 0x3_0010,
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
    /// Retrieve the VideoCore firmware revision
    FirmwareRevisionGet: {
        REQUEST: {},
        RESPONSE: {
            firmware_revision: u32
        }
    }
);

property_tag!(
    /// Retrieve the current board model
    BoardModelGet: {
        REQUEST: {},
        RESPONSE: {
            board_model: u32
        }
    }
);

property_tag!(
    /// Retrieve the board revision code. Refer to
    /// https://www.raspberrypi.org/documentation/hardware/raspberrypi/revision-codes/README.md for
    /// the encoding of it.
    BoardRevisionGet: {
        REQUEST: {},
        RESPONSE: {
            board_revision: u32
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

property_tag!(
    /// Retrieve the board serial number
    BoardSerialGet: {
        REQUEST: {},
        RESPONSE: {
            board_serial: u32
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
    /// Get the DMA channels that are usable
    /// Response:
    /// Bits 0-15  represents the DMA channels 0-15. If the corresponding bit is set for a
    /// channel it is usable. Bits 16-31 are reserved
    DmaChannelsGet: {
        REQUEST: {},
        RESPONSE: {
            channel_mask: u32
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

property_tag!(
    /// Retrieve the minimum available rate in Hz for a clock
    MinClockrateGet: {
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
    /// Retrieve the current voltage of the given [VoltageId]. The value represents an offset from
    /// 1.2V in units of 0.025V.
    VoltageGet: {
        REQUEST: {
            voltage_id: VoltageId
        },
        RESPONSE: {
            voltage_id: VoltageId,
            value: u32
        }
    }
);

property_tag!(
    /// Set the current voltage of the given [VoltageId]. The value represents an offset from
    /// 1.2V in units of 0.025V.
    VoltageSet: {
        REQUEST: {
            voltage_id: VoltageId,
            value: u32
        },
        RESPONSE: {
            voltage_id: VoltageId,
            value: u32
        }
    }
);

property_tag!(
    /// Retrieve the maximum supported voltage of the given [VoltageId]. The value represents an offset from
    /// 1.2V in units of 0.025V.
    MaxVoltageGet: {
        REQUEST: {
            voltage_id: VoltageId
        },
        RESPONSE: {
            voltage_id: VoltageId,
            value: u32
        }
    }
);

property_tag!(
    /// Retrieve the minimum supported voltage of the given [VoltageId]. The value represents an offset from
    /// 1.2V in units of 0.025V.
    MinVoltageGet: {
        REQUEST: {
            voltage_id: VoltageId
        },
        RESPONSE: {
            voltage_id: VoltageId,
            value: u32
        }
    }
);

property_tag!(
    /// Retrieve the current temperature in thousandths of a degree Celsius. The temperature id
    /// should always be passed as 0
    TemperatureGet: {
        REQUEST: {
            temperature_id: u32
        },
        RESPONSE: {
            temperature_id: u32,
            value: u32
        }
    }
);

property_tag!(
    /// Retrieve the maximum safe temperature in thousandths of a degree Celsius. The temperature id
    /// should always be passed as 0. Above this temperature the CPU might turn off overclock.
    MaxTemperatureGet: {
        REQUEST: {
            temperature_id: u32
        },
        RESPONSE: {
            temperature_id: u32,
            value: u32
        }
    }
);

property_tag!(
    /// Allocate a frame buffer with the given byte alignment
    FramebufferAllocate: {
        REQUEST: {
            alignment: u32
        },
        RESPONSE: {
            base_address: u32,
            size: u32
        }
    }
);

property_tag!(
    /// Release and disable the framebuffer
    FramebufferRelease: {
        REQUEST: {},
        RESPONSE: {}
    }
);

property_tag!(
    BlankScreen: {
        REQUEST: {
            state: u32
        },
        RESPONSE: {
            state: u32
        }
    }
);

property_tag!(
    /// Retrieve the physical display/framebuffer size which actually is the the size of the allocated
    /// frame buffer but usually larger than the displayed part that is passed to the monitor
    PhysicalSizeGet: {
        REQUEST: {},
        RESPONSE: {
            width: u32,
            height: u32
        }
    }
);

property_tag!(
    /// Set the physical display/framebuffer size which actually is the the size of the allocated
    /// frame buffer but usually larger than the displayed part that is passed to the monitor. The size
    /// returned by this tag might not match the requested size but is the closest supported one or
    /// 0 if no matching supported configuration could be determined.
    PhysicalSizeSet: {
        REQUEST: {
            width: u32,
            height: u32
        },
        RESPONSE: {
            width: u32,
            height: u32
        }
    }
);

property_tag!(
    /// Retreive the virtual display/framebuffer size. This is actually the size of the buffer passed to
    /// the monitor and might be only a part of the allocated frame buffer
    VirtualSizeGet: {
        REQUEST: {},
        RESPONSE: {
            width: u32,
            height: u32
        }
    }
);

property_tag!(
    /// Set the virtual display/framebuffer size. This is actually the size of the buffer passed to
    /// the monitor and might be only a part of the allocated frame buffer. The size returned by this
    /// tag might not match the requested size but is the closest supported one or 0 if no matching
    /// supported configuration could be determined.
    VirtualSizeSet: {
        REQUEST: {
            width: u32,
            height: u32
        },
        RESPONSE: {
            width: u32,
            height: u32
        }
    }
);

property_tag!(
    /// Retrieve the bits per pixel used for depth information
    DepthGet: {
        REQUEST: {},
        RESPONSE: {
            depth: u32
        }
    }
);

property_tag!(
    /// Set the bits per pixel used for depth information. If the provided depth value is not supported
    /// it will return 0 in the response.
    DepthSet: {
        REQUEST: {
            depth: u32
        },
        RESPONSE: {
            depth: u32
        }
    }
);

property_tag!(
    /// Retrieve the current pixel order. The returned value is:<br>
    /// 0x0 - BGR<br>
    /// 0x1 - RGB
    PixelOrderGet: {
        REQUEST: {},
        RESPONSE: {
            order: u32
        }
    }
);

property_tag!(
    /// Set the current pixel order. The possible values are:<br>
    /// 0x0 - BGR<br>
    /// 0x1 - RGB
    PixelOrderSet: {
        REQUEST: {
            order: u32
        },
        RESPONSE: {
            order: u32
        }
    }
);

property_tag!(
    /// Retrieve the current alpha mode. The possible values are:<br>
    /// 0x0 - alpha enabled, 0x0 in this channel of a pixel means full opaque
    /// 0x1 - alpha enabled, 0x0 in this channel of a pixel means full transparent
    /// 0x2 - alpha ignored, value in this channel of a pixel is ignored
    AlphaModeGet: {
        REQUEST: {},
        RESPONSE: {
            mode: u32
        }
    }
);

property_tag!(
    /// Retrieve the current alpha mode. The possible values are:<br>
    /// 0x0 - alpha enabled, 0x0 in this channel of a pixel means full opaque
    /// 0x1 - alpha enabled, 0x0 in this channel of a pixel means full transparent
    /// 0x2 - alpha ignored, value in this channel of a pixel is ignored
    /// If the rsponded mode differs from the requested it might not be supported
    AlphaModeSet: {
        REQUEST: {
            mode: u32
        },
        RESPONSE: {
            mode: u32
        }
    }
);

property_tag!(
    /// Retrieve the pitch in bytes per line
    PitchGet: {
        REQUEST: {},
        RESPONSE: {
            pitch: u32
        }
    }
);

property_tag!(
    /// Retrieve the current offset that is applied when retrieving the virtual display from the
    /// physical one.
    VirtualOffsetGet: {
        REQUEST: {},
        RESPONSE: {
            offset_x: u32,
            offset_y: u32
        }
    }
);

property_tag!(
    /// Set the current offset that is applied when retrieving the virtual display from the
    /// physical one.
    VirtualOffsetSet: {
        REQUEST: {
            offset_x: u32,
            offset_y: u32
        },
        RESPONSE: {
            offset_x: u32,
            offset_y: u32
        }
    }
);

property_tag!(
    OverscanGet: {
        REQUEST: {},
        RESPONSE: {
            top: u32,
            bottom: u32,
            left: u32,
            right: u32
        }
    }
);

property_tag!(
    OverscanSet: {
        REQUEST: {
            top: u32,
            bottom: u32,
            left: u32,
            right: u32
        },
        RESPONSE: {
            top: u32,
            bottom: u32,
            left: u32,
            right: u32
        }
    }
);

property_tag!(
    /// Retrieve the current palette color entries
    PaletteGet: {
        REQUEST: {},
        RESPONSE: {
            palette: [u32; 256]
        }
    }
);

property_tag!(
    /// Set/update the palette entries. As the palette buffer given is always a fixed sized array the
    /// offset need to be 0 and the length 256 and all palette colors need to be passed.
    PaletteSet: {
        REQUEST: {
            offset: u32,
            length: u32,
            palette: [u32; 256]
        },
        RESPONSE: {
            status: u32
        }
    }
);

property_tag!(
    /// Set the base address of the memory region used for the VCHIQ transmissions between ARM and GPU (VideoCore)
    VchiqInit: {
        REQUEST: {
            base_address: u32
        },
        RESPONSE: {
            status: u32
        }
    }
);
