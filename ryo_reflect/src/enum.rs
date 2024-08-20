use crate::reflect::Reflect;

pub trait Enum: Reflect {
    fn as_enum(&self) -> &dyn Enum;
    fn as_enum_mut(&self) -> &mut dyn Enum;
}