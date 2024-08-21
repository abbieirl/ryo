use core::ops::{Index, IndexMut};
use crate::reflect::Reflect;

pub trait Enum: Reflect {
    fn as_enum(&self) -> &dyn Enum;
    fn as_enum_mut(&self) -> &mut dyn Enum;

    fn variant(&self, name: &str) -> Option<&dyn Reflect>;
}

impl Index<usize> for dyn Enum {
    type Output = dyn Reflect;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl IndexMut<usize> for dyn Enum {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        todo!()
    }
}