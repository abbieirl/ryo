use raw_window_handle::HasWindowHandle;
use ryo_engine::{Engine, Module, Resource, Resources};

pub struct WindowModule;

#[derive(Default)]
pub struct WindowManager(Vec<Box<dyn HasWindowHandle + Send + Sync>>);

impl WindowManager {
    pub fn add_window(&mut self, handle: impl HasWindowHandle + Send + Sync + 'static) -> usize {
        let index = self.0.len();
        self.0.push(Box::new(handle));
        index
    }
}

impl Resource for WindowManager {}

impl Module for WindowModule {
    fn build(&self, _engine: &mut Engine) {
        Resources::insert(WindowManager::default());
    }
}
