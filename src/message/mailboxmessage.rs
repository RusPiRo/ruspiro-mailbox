/***************************************************************************************************
 * Copyright (c) 2020 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/

//! # Mailbox Message
//!
use crate::{propertytags::*, MessageState};

#[repr(C, align(16))]
pub(crate) struct MailboxMessage<T> {
    msg_size: u32,
    msg_type: MessageState,
    msg_tag: T,
    msg_end: u32,
}

impl<T> MailboxMessage<T>
where
    T: PropertyTag,
{
    /// Get the response from the property tag contained in this mailbox message after it has been
    /// processed
    pub fn response(&self) -> &T::Response {
        self.msg_tag.response()
    }

    /// Get the state of the processed mailbox message
    pub fn state(&self) -> MessageState {
        self.msg_type
    }
}

impl<T> From<T> for MailboxMessage<T>
where
    T: PropertyTag,
{
    /// Create a new mailbox message from a single property tag. This will be the only way a [MailboxMessage]
    /// can be constructed
    fn from(item: T) -> Self {
        MailboxMessage {
            msg_size: 12 + core::mem::size_of::<T>() as u32, //+ 8 + 4 + core::mem::size_of::<T>() as u32,
            msg_type: MessageState::Request,
            msg_tag: item,
            msg_end: 0x0,
        }
    }
}
