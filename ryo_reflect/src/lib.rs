#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

pub mod r#enum;
pub mod reflect;
pub mod r#struct;

#[cfg(feature = "rtti")]
pub mod r#type;

pub mod prelude {
    pub use crate::r#struct::Struct;
    pub use crate::reflect::Reflect;

    #[cfg(feature = "derive")]
    pub use crate::derive::*;
}

#[cfg(feature = "derive")]
pub mod derive {
    pub use ryo_reflect_derive::*;
}
