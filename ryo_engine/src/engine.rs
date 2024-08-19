use crate::{plugin::Plugin, runner::Runner};
use core::mem::take;

#[derive(Debug, Default)]
pub struct Engine {
    runner: Box<dyn Runner>,
}

impl Engine {
    pub fn add_plugin(&mut self, plugin: impl Plugin) -> &mut Self {
        plugin.build(self);
        self
    }

    pub fn add_system() {}

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
