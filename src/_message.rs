/***************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 ***************************************************************************************************/



/// Representation of a mailbox batch that can contain multiple property tags. The [MailboxBatch] can
/// only contain one entry for each [PropertyTag] type.
#[repr(C, align(16))]
pub struct MailboxBatch {
    pub(crate) buffer: Vec<u32>,
    pub(crate) tag_offsets: BTreeMap<TypeId, u32>,
}

impl MailboxBatch {
    pub fn empty() -> Self {
        MailboxBatch {
            // buffer always starts with 2 u32 values.
            // The first is the placeholder for the final batch message size and it starts with 12
            // containing the batch header(type+size each u32) + a closing u32
            // The second represent the message type
            buffer: vec![12, MessageState::Request as u32],
            tag_offsets: BTreeMap::new(),
        }
    }

    pub fn add_tag<T: PropertyTag + 'static>(&mut self, tag: T) -> MailboxResult<()> {
        if self.tag_offsets.contains_key(&TypeId::of::<T>()) {
            return Err("duplicate property tag in batch is not allowed");
        }
        // get the size of the tag to be added to the batch
        let tag_size = core::mem::size_of::<T>();
        // get the &[u32] representation of the property tag
        // this is save as every property tag need to be always a size that is a multiple of the
        // size of an u32
        let slice =
            unsafe { core::slice::from_raw_parts(&tag as *const T as *const u32, tag_size >> 2) };
        // store the offset in the buffer this message is added to
        self.tag_offsets
            .insert(TypeId::of::<T>(), self.buffer.len() as u32);
        self.buffer.extend_from_slice(slice);
        self.buffer[0] += tag_size as u32;
        Ok(())
    }

    pub fn get_tag<T: PropertyTag + 'static>(&self) -> Option<&T> {
        // get the offset of this tag type if it is stored
        let offset = self.tag_offsets.get(&TypeId::of::<T>())?;
        // "cast" the buffer into the Tag structure
        let tag = unsafe {
            let ptr = &self.buffer[*offset as usize] as *const u32 as *const T;
            &*ptr as &T
        };

        Some(tag)
    }
}
