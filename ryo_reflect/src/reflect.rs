#[cfg(feature = "rtti")]
use crate::r#type::{Type, TypeInfo};

use core::any::{type_name, Any};
use std::sync::LazyLock;

pub trait Reflect: Any {
    /// Returns the name of this type as a string slice.
    fn type_name(&self) -> &'static str;

    /// Returns the path of this type as a string slice.
    #[inline]
    fn type_path(&self) -> &'static str {
        type_name::<Self>()
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_reflect(&self) -> &dyn Reflect;
    fn as_reflect_mut(&mut self) -> &mut dyn Reflect;
}

impl dyn Reflect {
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
        static _RTTI: LazyLock<TypeInfo> = LazyLock::new(Default::default);

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
impl_reflect!(char, &'static str);
impl_reflect!(bool);
impl_reflect!(());
