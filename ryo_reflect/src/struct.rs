use crate::reflect::Reflect;
use core::fmt::{Debug, Formatter};

pub trait Struct: Reflect {
    fn as_struct(&self) -> &dyn Struct;
    fn as_struct_mut(&mut self) -> &mut dyn Struct;
    fn field(&self, name: &str) -> Option<&dyn Reflect>;
    fn field_mut(&mut self, name: &str) -> Option<&mut dyn Reflect>;
    fn field_index(&self, index: usize) -> Option<&dyn Reflect>;
    fn field_index_mut(&mut self, index: usize) -> Option<&mut dyn Reflect>;
    fn field_count(&self) -> usize;
    fn field_name(&self, index: usize) -> Option<&'static str>;
    fn field_value(&self, index: usize) -> Option<&dyn Reflect>;
}

impl Debug for dyn Struct {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut debug = f.debug_struct(self.type_name());

        (0..self.field_count()).into_iter().for_each(|index| {
            // SAFETY: index is in range
            let name = unsafe { self.field_name(index).unwrap_unchecked() };
            let value = unsafe { self.field_value(index).unwrap().as_any() };

            debug.field(name, &value as &dyn Debug);
        });

        debug.finish()
    }
}
