use crate::reflect::Reflect;
use core::any::TypeId;

pub trait Struct: Reflect {
    fn field(&self, name: &str) -> Option<&dyn Reflect>;

    fn field_mut(&mut self, name: &str) -> Option<&mut dyn Reflect>;

    fn field_idx_mut(&mut self, idx: usize) -> Option<&mut dyn Reflect>;

    fn field_count(&self) -> usize;
}

#[derive(Debug)]
pub struct StructData {
    type_id: TypeId,
    type_name: &'static str,
}

impl StructData {
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn type_name(&self) -> &'static str {
        self.type_name
    }
}

#[derive(Debug)]
pub struct FieldData {
    type_id: TypeId,
    name: &'static str
}