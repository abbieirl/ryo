use core::any::{type_name_of_val, TypeId};
use core::fmt::{Debug, Formatter, Result};
use core::any::Any;
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

static RESOURCES: LazyLock<Resources> = LazyLock::new(|| Resources::default());

#[derive(Debug, Default)]
pub struct Resources(RwLock<HashMap<TypeId, Box<dyn Resource>>>);

impl Resources {
    pub fn insert<R: Resource>(resource: R) {
        let mut resources = RESOURCES.0.write().unwrap();
        resources.insert(TypeId::of::<R>(), Box::new(resource));
    }

    pub fn get<R: Resource>() -> Option<&'static R> {
        let resources = RESOURCES.0.read().unwrap();
        resources
            .get(&TypeId::of::<R>())
            .map(|resource| unsafe { &*(&**resource as *const _ as *const R) })
    }

    pub fn get_mut<R: Resource>() -> Option<&'static mut R> {
        let mut resources = RESOURCES.0.write().unwrap();
        resources
            .get_mut(&TypeId::of::<R>())
            .map(|resource| unsafe { &mut *(&mut **resource as *mut _ as *mut R) })
    }
}

pub trait Resource: 'static + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl Debug for dyn Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(type_name_of_val(self))
    }
}
