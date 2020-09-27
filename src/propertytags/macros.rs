/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/

//! # Macro definitions to provide syntactic sugar when declaring a new property tag message structure
//!

/// This macro defines the request part of the property tag data
/// The macro expands to:
/// ```
/// #[repr(C)]
/// #[derive(Copy, Clone)]
/// pub struct <PropertyTagName>DataRequest {
///     <field list>
/// }
/// ```
macro_rules! property_tag_request {
    ($name:ident, {$($field:ident:$type:ty), *}) => {
        #[doc(hidden)]
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct $name {
            $(
                $field: $type,
            )*
        }
    }
}

/// This macro defines the response part of the property tag data
/// The macro expands to:
/// ```ignore
/// #[repr(C)]
/// #[derive(Copy, Clone)]
/// pub struct <PropertyTagName>DataResponse {
///     <field list>
/// }
/// ```
macro_rules! property_tag_response {
    ($name:ident, {$($field:ident:$type:ty), *}) => {
        #[doc(hidden)]
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct $name {
            $(
                pub $field: $type,
            )*
        }

        #[doc(hidden)]
        impl $name {
            $(
                pub fn $field(&self) -> $type {
                    self.$field
                }
            )*
        }
    }
}

/// This macros defines the message part of the property tag and will contain the request and response
/// The macro expansd to:
/// ```ignore
/// #[repr(C)]
/// pub union <PropertyTagName>Data {
///     request: <PropertyTagName>DataRequest,
///     response: <PropertyTagName>DataResponse,
/// }
/// ```
macro_rules! property_tag_data {
    ($name:ident, $req_fields:tt, $rsp_fields:tt) => {
        paste::item! {
                    #[doc(hidden)]
                    #[repr(C)]
                    #[derive(Copy, Clone)]
                    pub union $name {
                        request: [<$name Request>],
                        response: [<$name Response>],
                    }

                    property_tag_request!([<$name Request>], $req_fields);
                    property_tag_response!([<$name Response>], $rsp_fields);
        /*
                    impl core::fmt::Debug for $name {
                        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                            write!(f, "{:?}", unsafe { self.response })
                        }
                    }
        */
                }
    };
}

/// Simple helper macro to create the right initialization of the padding field
macro_rules! init_padding {
    (u8) => {
        0
    };
    (u16) => {
        0
    };
    ([u8; $size:expr]) => {
        [0; $size]
    };
}

/// This macros defines the required implementation for the property tag message structure defined so far
macro_rules! property_tag_impl {
    ($name:ident, {$($field:ident:$type:ty),*} $(,$padding:ident:$padtype:ty)?) => {
        paste::item! {
            impl PropertyTag for $name {
                type Request = [<$name Data Request>];
                type Response = [<$name Data Response>];

                /// Get the [PropertyTagId] of this property tag
                fn tagid(&self) -> PropertyTagId {
                    self.tagid
                }

                /// Get the state of this property tag. This is usually requested after the response
                /// from the mailbox has been received to see if this tag has been successfully
                /// processesed
                fn state(&self) -> u32 {
                    self.tagstate
                }

                /// Retrieve the response from the mailbox property tag.
                /// # Safety
                /// As the ``Request`` and the ``Response`` share the same memory region this is
                /// designed as a union structure. Access to ``Response`` part of this union is
                /// safe after the message has been retrieved from the mailbox as this is the contract
                /// of the mailbox specification we adhere to.
                fn response(&self) -> &Self::Response {
                    unsafe { &self.tagdata.response }
                }

                fn size(&self) -> u32 {
                    self.tagsize
                }
            }

            #[allow(clippy::new_without_default)]
            impl $name {
                /// Create a new property tag. All data the property tag defines in it's Request
                /// structure are expected to be provided when constructing a [PropertyTag]
                pub fn new(
                    $(
                        [<$field _val>]: $type,
                    )*
                ) -> Self {
                    Self {
                        tagid: PropertyTagId::$name,
                        tagsize: core::mem::size_of::<[<$name Data>]>() as u32,
                        tagstate: 0x0,
                        tagdata: [<$name Data>] {
                            request: [<$name Data Request>] {
                                $(
                                    $field: [<$field _val>],
                                )*
                            }
                        },
                        $($padding: init_padding!($padtype),)?
                    }
                }
            }
        }
    };
}

/// Helper macro to conviniently define a [PropertyTag] structure
///
/// # Examples
/// ```no_run
/// # use ruspiro_mailbox::*;
/// property_tag! {
///     ClockrateGet: {
///         REQUEST: {
///             clockId: u32
///         },
///         RESPONSE: {
///             clockId: u32,
///             clockRate: u32
///         }
///     }
/// }
///
/// # fn doc() {
/// // create a new clock rate get message from the respective property tag
/// let message = MailboxMessage::from_property_tag(
///     ClockrateGet::new(ClockId::Arm)
/// );
/// # }
/// ```
/// If MAX(size_of(REQUEST), size_of(RESPONSE)) is not a multiple of size_of(u32) than padding need
/// to be added to the property tag like so:
/// ```
/// # use ruspiro_mailbox::property_tag;
/// property_tag!(
///     BoardMacAddressGet: {
///         REQUEST: {
///         },
///         RESPONSE: {
///             address: [u8;6]
///         },
///         PADDING: [u8;2]
///     }
/// );
/// ```
/// Padding is available with the following variants: ``PADDING: u8``, ``PADDING: [u8; x]`` or ``PADDING: u16``.
///
/// The constructor ``new`` of the property tag contains all parameters of the tag request structure
/// that need to be passed to create a valid property tag.
/// The resulting [PropertyTag] can be used to be converted into a [MailboxMessage] and send to the
/// mailbox or it can be added to a [MailboxBatch] to be send with additional [PropertyTag]s.
macro_rules! property_tag {
    ($(#[doc = $doc:expr])* $name:ident : { REQUEST: $req_fields:tt , RESPONSE: $rsp_fields:tt $(, PADDING:$type:ty)?}) => {
        paste::item! {
            $(#[doc = $doc])*
            #[allow(dead_code)]
            #[repr(C, packed)]
            #[derive(Copy, Clone)]
            pub struct $name {
                /// Property Tag Id. See [PropertyTagId]
                tagid: PropertyTagId,
                /// The payload size of this property tag
                tagsize: u32,
                /// The state of this property tag. This should be 0x0 on request. On response
                /// bit 31 is set to 1 and bit 30..0 contains the response payload size as it would
                /// be from the sender point of view. This value could be larger than the actual passed
                /// paylload buffer size. In this case the response is truncated to fit into the provided
                /// buffer
                tagstate: u32,
                /// property tag data need to be aligned to 32 bits
                tagdata: [<$name Data>],
                $(tagpadding: $type,)?
            }

            property_tag_data!([<$name Data>], $req_fields, $rsp_fields);
            property_tag_impl!($name, $req_fields $(,tagpadding:$type)?);
        }
    };
}
