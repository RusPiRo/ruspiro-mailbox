/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/

//! # Low-level mailbox property tag interface
//!
//! This module provide the low level implementation of the mailbox property tag interface dealing with the actual
//! peripherals.
//!
use crate::{
    MailboxBatch, MailboxChannel, MailboxMessage, MailboxResult, MessageState, PropertyTag,
};
use ruspiro_cache as cache;
use ruspiro_register::define_mmio_register;

// MMIO base address for peripherals
#[cfg(feature = "ruspiro_pi3")]
const PERIPHERAL_BASE: u32 = 0x3F00_0000;

// Mailbox MMIO base address
const MAILBOX_BASE: u32 = PERIPHERAL_BASE + 0x0000_B880;

/// Function to send a specific message to the mailbox channel given
/// The mailbox interface does update the memory location of the message send. Therefor the function
/// returns ``Ok(_)`` with the updated message in case of a success
#[inline(never)]
pub(crate) fn send_message<T: PropertyTag>(
    channel: MailboxChannel,
    mut message: MailboxMessage<T>,
) -> MailboxResult<MailboxMessage<T>> {
    let msg_ptr: *mut MailboxMessage<T> = &mut message;
    let msg_ptr_uncached: u32 = (msg_ptr as u32) | 0xC000_0000;

    cache::cleaninvalidate();
    mb_write(channel, msg_ptr_uncached)?;
    mb_read(channel)?;
    cache::cleaninvalidate();
    // at this point the property tag message memory has been changed under the hood
    // that Rust is not aware of, so optimizations might do things that will loose this fact
    // so read this memory location back into the corresponding message type
    let result_ptr = (msg_ptr_uncached ^ 0xC000_0000) as *mut MailboxMessage<T>;
    let result = unsafe { core::ptr::read_volatile(result_ptr) };
    // now that we have reconstructed the MailboxMessage from the exact memory location
    // as the prevoius one, ensure the previous one will not being dropped as this might release
    // resources now used be the reconstructed version
    core::mem::forget(message);

    match result.state() {
        MessageState::ResponseOk => Ok(result),
        _ => Err("unable to send mailbox property tag message."),
    }
}

#[inline(never)]
pub(crate) fn send_batch<T>(
    channel: MailboxChannel,
    mut batch: MailboxBatch<T>,
) -> MailboxResult<MailboxBatch<T>> {
    // get the binary data from the batch and pass the address to it to the mailbox for processing
    let batch_ptr: *mut u32 = &mut batch as *mut MailboxBatch<T> as *mut u32;
    let batch_ptr_uncached: u32 = (batch_ptr as u32) | 0xC000_0000;
    // send this mailbox message and wait for the GPU to respond
    cache::cleaninvalidate();
    mb_write(channel, batch_ptr_uncached)?;
    mb_read(channel)?;
    cache::cleaninvalidate();

    // at this point the property tag message batch memory has been changed under the hood
    // that Rust is not aware of, so optimizations might do things that will loose this fact
    // so read this memory location back into the corresponding buffer type
    let result_ptr = (batch_ptr_uncached ^ 0xC000_0000) as *mut MailboxBatch<T>;
    let result = unsafe { core::ptr::read_volatile(result_ptr) };
    // as we have reconstructed the MailboxBatch at the exact location of the previous one
    // we need to ensure the previous one does not get dropped as this might release
    // resources now used be the reconstructed version
    core::mem::forget(batch);
    if let MessageState::ResponseOk = result.msg_type {
        Ok(result)
    } else {
        Err("unable to send mailbox property tag message.")
    }
}
/*
pub(crate) fn send_batch2(
    channel: MailboxChannel,
    mut batch: MailboxBatch,
) -> MailboxResult<MailboxBatch> {
    // before sending push the closing u32 as tag-end marker to the batch
    batch.buffer.push(0x0);
    // get the binary data from the batch and pass the address to it to the mailbox for processing
    let batch_ptr: *mut u32 = batch.buffer.as_mut_ptr();
    let batch_ptr_uncached: u32 = (batch_ptr as u32) | 0xC000_0000;
    // send this mailbox message and wait for the GPU to respond
    cache::cleaninvalidate();
    mb_write(channel, batch_ptr_uncached)?;
    mb_read(channel)?;
    cache::cleaninvalidate();

    // at this point the property tag message batch memory has been changed under the hood
    // that Rust is not aware of, so optimizations might do things that will loose this fact
    // so read this memory location back into the corresponding buffer type
    let result_ptr = (batch_ptr_uncached ^ 0xC000_0000) as *mut u32;
    let result = unsafe { Vec::from_raw_parts(result_ptr, batch.buffer.len(), batch.buffer.len()) };

    if result[1] == MessageState::ResponseOk as u32 {
        // as we have rconstructed the buffer vector at the exact location of the previous one
        // we nee to ensure the previous one does not get dropped as this might release
        // resources now used be the reconstructed version
        core::mem::forget(batch.buffer);

        // re-construct a MailboxBatch from the data to return it from this function
        let batch = MailboxBatch {
            buffer: result,
            // we can re-use the previous tag offsets as their position cannot change during GPU processing
            tag_offsets: batch.tag_offsets,
        };

        Ok(batch)
    } else {
        Err("unable to send mailbox property tag message.")
    }
}
*/

define_mmio_register! [
    MAILBOX0_READ<ReadOnly<u32>@(MAILBOX_BASE)>,
    MAILBOX0_STATUS<ReadOnly<u32>@(MAILBOX_BASE + 0x18)>,
    MAILBOX1_WRITE<WriteOnly<u32>@(MAILBOX_BASE + 0x20)>,
    MAILBOX1_STATUS<ReadOnly<u32>@(MAILBOX_BASE + 0x38)>
];

const MAILBOX_FULL: u32 = 0x8000_0000; // status register value if the mailbox is already full
const MAILBOX_EMPTY: u32 = 0x4000_0000; // status register value if the mailbox is empty

#[inline]
fn mb_read(channel: MailboxChannel) -> MailboxResult<u32> {
    loop {
        while (MAILBOX0_STATUS::Register.get() & MAILBOX_EMPTY) != 0x0 {}
        let data = MAILBOX0_READ::Register.get();
        if (data & 0xF) == channel as u32 {
            return Ok(data & 0xFFFF_FFF0);
        }
    }
}

#[inline]
fn mb_write(channel: MailboxChannel, data: u32) -> MailboxResult<()> {
    while (MAILBOX1_STATUS::Register.get() & MAILBOX_FULL) != 0x0 {}
    let value = (data & 0xFFFF_FFF0) | ((channel as u8) & 0xF) as u32;
    MAILBOX1_WRITE::Register.set(value);
    Ok(())
}
