#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[cfg(feature = "rtti")]
use crate::r#type::{Type, TypeInfo};

use crate::r#struct::Struct;
use core::any::{type_name_of_val, Any};
use core::cell::LazyCell;
use core::fmt::{Debug, Formatter};

pub trait Reflect: Any {
    #[cfg(feature = "alloc")]
    fn into_any(self: Box<Self>) -> Box<dyn Any>;

    fn type_name(&self) -> &'static str {
        type_name_of_val(self)
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_reflect(&self) -> &dyn Reflect;
    fn as_reflect_mut(&mut self) -> &mut dyn Reflect;
}

impl dyn Reflect {
    // #[cfg(feature = "alloc")]
    // pub fn downcast<T: Reflect>(self: Box<dyn Reflect>) -> Result<Box<T>, Box<Self>> {
    //     self.into_any()
    //         .downcast()
    //         .map_err(|any| any as Box<dyn Reflect>)
    // }

    #[inline]
    pub fn downcast_ref<T: Reflect>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }

    #[inline]
    pub fn downcast_mut<T: Reflect>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }
}

impl Debug for dyn Reflect {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.type_name())
    }
}

#[cfg(feature = "rtti")]
impl Type for dyn Reflect {
    fn type_info() -> &'static TypeInfo {
        // struct Wrap(u8);
        // unsafe impl Sync for Wrap {}
        // static RTTI: LazyCell<Wrap1> = LazyCell::new(Default::default);
        todo!()
    }
}

impl<T> Reflect for T
where
    T: Struct,
{
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn as_reflect(&self) -> &dyn Reflect {
        self
    }

    fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        self
    }
}

macro_rules! impl_reflect {
    ($($t:ty),*) => {
        $(
            impl Reflect for $t {
                fn into_any(self: Box<Self>) -> Box<dyn Any> {
                    self
                }

                fn as_any(&self) -> &dyn Any {
                    self as &dyn Any
                }

                fn as_any_mut(&mut self) -> &mut dyn Any {
                    self as &mut dyn Any
                }

                fn as_reflect(&self) -> &dyn Reflect {
                    self
                }

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
