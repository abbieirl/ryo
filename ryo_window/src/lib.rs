use ryo_engine::{Engine, Module, Resource, Resources};

pub struct WindowModule;

pub struct WindowManager;

impl Resource for WindowManager {}

impl Module for WindowModule {
    fn build(&self, _engine: &mut Engine) {
        Resources::insert(WindowManager);
    }
}
