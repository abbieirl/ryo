use crate::component::Component;
use crate::entity::Entity;
use crate::system::System;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::any::TypeId;
use hashbrown::HashMap;

#[derive(Debug, Default)]
pub struct World {
    entities: Vec<usize>,
    systems: Vec<Arc<dyn System>>,
    components: HashMap<TypeId, Vec<Arc<dyn Component>>>,
}

impl World {
    pub fn add_component<C: Component>(&mut self, entity: Entity, component: C) {
        let type_id = TypeId::of::<C>();

        self.components
            .entry(type_id)
            .or_insert_with(Vec::new)
            .insert(entity.index() as usize, Arc::new(component));
    }

    pub fn get_component<C: Component>(&mut self) -> Option<Arc<dyn Component>> {
        todo!()
    }

    pub fn add_system(&mut self, system: impl System) {
        self.systems.push(Arc::new(system));
    }

    pub fn run_system(&mut self) {}
}
