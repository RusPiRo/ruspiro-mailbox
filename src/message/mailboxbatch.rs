/***************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **************************************************************************************************/

//! # MailboxBatch message
//!

use crate::propertytags::*;
use crate::MessageState;

/// The wrapper storing all property tags that comprises a batch message
#[derive(Debug)]
#[repr(C, align(16))]
pub struct MailboxBatch<Tags> {
    msg_size: u32,
    pub msg_type: MessageState,
    msg_tags: Tags,
    msg_end: u32,
}

/// Define a trait that allows to constrain the ``Tags`` generics used with the [MailboxBatch]
pub trait PropertyTagList {}

/// Define linked list of tags contained in the [MailboxBatch]. This is a compiletime only list. On
/// memory the different concecutive tags exists as packed array (hopefully)
#[derive(Debug, Clone, Copy)]
pub struct Cons<Prev, Tag> {
    previous: Prev,
    tag: Tag,
}

/// Define the 'Empty' batch
#[derive(Debug)]
pub struct Empty;

/// Implement the [PropertyTagList] trait for the Cons structure.
impl<Prev: PropertyTagList, Tag: PropertyTag> PropertyTagList for Cons<Prev, Tag> {}

/// Implement the [PropertyTagList] trait for the Empty batch variant
impl PropertyTagList for Empty {}

/// provide the function to create an empty [MailboxBatch] for the [Empty] type only
impl MailboxBatch<Empty> {
    pub fn empty() -> Self {
        Self {
            msg_size: 12,
            msg_type: MessageState::Request,
            msg_tags: Empty,
            msg_end: 0,
        }
    }
}

/// provide the function to add a new tag to the batch for all types of the batch that
/// implement the PropertyTagList trait
impl<Tags: PropertyTagList> MailboxBatch<Tags> {
    /// adding a new tag to the batch means we create a new type that contains the new tag at the end
    /// of the linked list. The type Tags will be converted to Cons<Tags, Tag>
    /// The current MailboxBatch is consumed by this call and a new one returned, kind of similar to
    /// a builder pattern ?
    /// As the tags are concecutive in the linked list they are also layed out concecutive in the memory
    /// as we require it to happen, the batch header part and the final u32 are kept in place...
    pub fn add_tag<Tag>(self, tag: Tag) -> MailboxBatch<Cons<Tags, Tag>> {
        MailboxBatch {
            msg_size: self.msg_size + core::mem::size_of::<Tag>() as u32,
            msg_type: self.msg_type,
            msg_tags: Cons {
                previous: self.msg_tags,
                tag,
            },
            msg_end: self.msg_end,
        }
    }

    /// The tricky part to find a tag after it has been added based on it's type
    pub fn find<Tag, Pos>(&self) -> &Tag
    where
        Tags: FindTag<Tag, Pos>,
    {
        self.msg_tags.find()
    }
}

/// A trait that defines that it can find a tag of a specified type in the linked list Cons
pub trait FindTag<Tag, Pos> {
    fn find(&self) -> &Tag;
}

/// Positions where we would like to find the correct type
pub struct Here;
pub struct Next<T>(T);

/// Implement the find trait for the Cons<_,_> structure that is only available if the types of the
/// Tag we would like to access and the tag type in the Cons<_, _> matches at position [Here]. This
/// is a "compiletime" find in the linked list for the actual position
impl<Prev, Tag> FindTag<Tag, Here> for Cons<Prev, Tag> {
    fn find(&self) -> &Tag {
        &self.tag
    }
}

/// Implement the find trait for the Cons<_,_> structure that is only available if the types of the
/// Tag we would like to access matches the actual type of the previous entry of Cons<_, _>
impl<Prev: FindTag<Tag, Pos>, Tag, Pos, T> FindTag<Tag, Next<Pos>> for Cons<Prev, T> {
    fn find(&self) -> &Tag {
        self.previous.find()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClockId;

    #[test]
    fn create_single_item_batch() {
        let batch = MailboxBatch::empty().add_tag(ClockrateGet::new(ClockId::Core));

        println!("Batch: {:#?}", batch);
        let slice = unsafe {
            core::slice::from_raw_parts(
                &batch as *const MailboxBatch<_> as *const u32,
                (batch.msg_size >> 2) as usize,
            )
        };
        println!("message binary: {:#X?}", slice);

        let _ = batch.find::<ClockrateGet, _>();
    }

    #[test]
    fn create_multiple_item_batch() {
        let batch = MailboxBatch::empty()
            .add_tag(ClockrateGet::new(ClockId::Core))
            .add_tag(MaxClockrateGet::new(ClockId::Arm))
            .add_tag(BoardMACAddressGet::new());

        println!("Batch: {:#?}", batch);
        let slice = unsafe {
            core::slice::from_raw_parts(
                &batch as *const MailboxBatch<_> as *const u32,
                (batch.msg_size >> 2) as usize,
            )
        };
        println!("message binary: {:#X?}", slice);

        let _ = batch.find::<MaxClockrateGet, _>();
    }
}
