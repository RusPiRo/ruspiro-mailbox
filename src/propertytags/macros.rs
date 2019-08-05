/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Apache License 2.0
 **********************************************************************************************************************/

//! # Macro definitions to provide syntactic sugar when declaring a new property tag message structure
//! 

/// This macro defines the request part of the property tag message data
/// The macro expands to:
/// ```
/// #[repr(C)]
/// #[derive(Copy, Clone)]
/// pub struct <PropertyTag>DataRequest {
///     <field list>
/// }
/// ```
macro_rules! property_tag_msg_request {
    ($name:ident, [$($field:ident:$type:ty), *]) => {
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct $name {
            $(
                $field: $type,
            )*
        }
    }
}

/// This macro defines the response part of the property tag message data
/// The macro expands to:
/// ```
/// #[repr(C)]
/// #[derive(Copy, Clone)]
/// pub struct <PropertyTag>DataResponse {
///     <field list>
/// }
/// ```
macro_rules! property_tag_msg_response {
    ($name:ident, [$($field:ident:$type:ty), *]) => {
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct $name {
            $(
                pub $field: $type,
            )*
        }
    }
}

/// This macros defines the message part of the property tag and will contain the request and response
/// 
macro_rules! property_tag_msg_data {
    ($name:ident, $req_fields:tt, $rsp_fields:tt) => {
        #[repr(C)]
        paste::item! {
            pub union $name {
                request: [<$name Request>],
                pub response: [<$name Response>]
            }
        
            property_tag_msg_request!([<$name Request>], $req_fields);
            property_tag_msg_response!([<$name Response>], $rsp_fields);
        }
    };
}

/// This macros defines the required implementation for the property tag message structure defined so far
/// 
macro_rules! property_tag_msg_impl {
    ($name:ident, [$($field:ident:$type:ty),*]) => {
        paste::item! {
            impl MailboxMessage for $name {
                fn get_state(&self) -> u32 {
                    self.msg_type
                }
            }

            impl $name {
                pub fn new(
                    $(
                        [<$field _val>]: $type,
                    )*
                ) -> Self {
                    Self {
                        msg_size: 12 + 8 + 4 + core::mem::size_of::<[<$name Data>]>() as u32,
                        msg_type: MessageState::Request as u32,
                        msg_tagid: PropertyTag::$name,
                        msg_tagsize: core::mem::size_of::<[<$name Data>]>() as u32,
                        msg_tagstate: 0x0,
                        msg_tagdata: [<$name Data>] {
                            request: [<$name Data Request>] {
                                $(
                                    $field: [<$field _val>],
                                )*
                            }
                        },
                        msg_end: 0x0
                    }
                }

                pub fn get_response(&self) -> [<$name Data Response>] {
                    unsafe { self.msg_tagdata.response }
                }
            }
        }
    };
}

/// Helper macro to conviniently define a property tag message structure
/// 
/// # Examples
/// 
/// 
/// ```
/// # use rubo_mailbox::property_tag_message
/// property_tag_message! {
/// ClockrateGet:
///     REQUEST [
///         clockId: u32
///     ]
///     RESPONSE [
///         clockId: u32,
///         clockRate: u32
///     ]
/// }
/// 
/// # fn main() {
/// let clockrage_msg = ClockrateGet::new(0x0001);
/// # }
/// ```
/// The constructor **new** of the property tag message contains all parameters of the message request that need to be
/// passed to create the message. The created message is always at a 32bit aligned memory address and could be
/// immediately used with the send function of the mailbox interface
//#[macro_export]
macro_rules! property_tag_message {
    ($name:ident : REQUEST $req_fields:tt RESPONSE $rsp_fields:tt) => {

        paste::item!{
            #[allow(dead_code)]
            #[repr(C, align(16))]
            pub struct $name {
                // message header                
                msg_size: u32,
                msg_type: u32,
                // tag header
                msg_tagid: PropertyTag,
                msg_tagsize: u32,
                msg_tagstate: u32,
                // tag data
                pub msg_tagdata: [<$name Data>],
                // closing word to be set to 0
                msg_end: u32
            }

            property_tag_msg_data!([<$name Data>], $req_fields, $rsp_fields);
            property_tag_msg_impl!($name, $req_fields);
        }
    };
}