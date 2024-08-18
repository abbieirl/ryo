#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod r#enum;
pub mod reflect;
pub mod r#struct;

#[cfg(feature = "rtti")]
pub mod r#type;

pub mod prelude {
    pub use crate::reflect::Reflect;
    pub use crate::r#struct::Struct;
}

#[cfg(feature = "derive")]
pub use ryo_reflect_derive::Reflect;
