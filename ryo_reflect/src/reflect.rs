#[cfg(feature = "std")]
use std::sync::LazyLock;

#[cfg(feature = "rtti")]
use crate::r#type::{Type, TypeInfo};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use core::any::{type_name, Any, TypeId};

pub trait Reflect: Any {
    /// Returns the name of this type as a string slice.
    fn type_name(&self) -> &'static str;

    /// Returns the path of this type as a string slice.
    #[inline]
    fn type_path(&self) -> &'static str {
        type_name::<Self>()
    }

    #[cfg(feature = "alloc")]
    fn into_any(self: Box<Self>) -> Box<dyn Any>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_reflect(&self) -> &dyn Reflect;
    fn as_reflect_mut(&mut self) -> &mut dyn Reflect;
}

impl dyn Reflect {
    pub fn is<T: Reflect>(&self) -> bool {
        self.type_id() == TypeId::of::<T>()
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn downcast<T: Reflect>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
        if self.is::<T>() {
            Ok(unsafe { self.into_any().downcast().unwrap_unchecked() })
        } else {
            Err(self)
        }
    }

    #[inline]
    pub fn downcast_ref<T: Reflect>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }

    #[inline]
    pub fn downcast_mut<T: Reflect>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }
}

#[cfg(feature = "rtti")]
impl Type for dyn Reflect {
    fn type_info() -> &'static TypeInfo {
        #[cfg(feature = "std")]
        static _RTTI: LazyLock<u8> = LazyLock::new(Default::default);

        #[cfg(all(feature = "alloc", not(feature = "std")))]
        static _RTTI: LazyCell<u8> = LazyCell::new(Default::default);

        todo!()
    }
}

macro_rules! impl_reflect {
    ($($t:ty),*) => {
        $(
            impl Reflect for $t {
                #[inline]
                fn type_name(&self) -> &'static str {
                    stringify!($t)
                }

                #[inline(always)]
                #[cfg(feature = "alloc")]
                fn into_any(self: Box<Self>) -> Box<dyn Any> {
                    self
                }

                #[inline(always)]
                fn as_any(&self) -> &dyn Any {
                    self as &dyn Any
                }

                #[inline(always)]
                fn as_any_mut(&mut self) -> &mut dyn Any {
                    self as &mut dyn Any
                }

                #[inline(always)]
                fn as_reflect(&self) -> &dyn Reflect {
                    self
                }

                #[inline(always)]
                fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
                    self
                }
            }
        )*
    };
}

impl_reflect!(i8, i16, i32, i64, i128, isize);
impl_reflect!(u8, u16, u32, u64, u128, usize);
impl_reflect!(f32, f64);
impl_reflect!(char);
impl_reflect!(bool);
impl_reflect!(());
