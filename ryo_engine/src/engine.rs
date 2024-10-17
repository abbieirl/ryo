use crate::{Module, Runner};
use core::mem::take;
use ryo_ecs::system::System;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct Engine {
    runner: Box<dyn Runner>,
    pub(crate) systems: Vec<Arc<dyn System>>,
}

impl Engine {
    pub fn add_module(&mut self, module: impl Module) -> &mut Self {
        module.build(self);
        self
    }

    pub fn add_system(&mut self, system: impl System) -> &mut Self {
        self.systems.push(Arc::new(system));
        self
    }

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
