use crate::{Module, Resource, Runner};
use core::mem::take;
use ryo_ecs::system::System;

#[derive(Debug, Default)]
pub struct Engine {
    runner: Box<dyn Runner>,
}

impl Engine {
    #[inline]
    pub fn add_module(&mut self, module: impl Module) -> &mut Self {
        module.build(self);
        self
    }

    #[inline]
    pub fn add_system(&mut self, system: impl System) -> &mut Self {
        let _ = system;
        self
    }

    #[inline]
    pub fn add_resource(&mut self, resource: impl Resource) -> &mut Self {
        let _ = resource;
        self
    }

    #[inline]
    pub fn get_resource(&mut self, resource: impl Resource) -> &mut Self {
        let _ = resource;
        self
    }

    #[inline]
    pub fn set_runner(&mut self, runner: impl Runner) -> &mut Self {
        self.runner = Box::new(runner);
        self
    }

    pub fn run(&mut self) {
        let mut engine = take(self);
        let runner = take(&mut engine.runner);

        runner.run(engine);
    }
}
