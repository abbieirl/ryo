use raw_window_handle::HasWindowHandle;
use ryo_engine::{Engine, Module, Resource, Resources};

#[derive(Debug, Default)]
pub struct WindowModule {
    windows: Vec<Window>,
}

impl WindowModule {
    #[inline]
    pub fn with_window(mut self, window: Window) -> Self {
        self.windows.push(window);
        self
    }
}

#[derive(Default)]
pub struct WindowManager(pub Vec<(Window, Option<Box<dyn HasWindowHandle + Send + Sync>>)>);

impl Resource for WindowManager {}

impl Module for WindowModule {
    fn build(&self, _engine: &mut Engine) {
        let windows = self
            .windows
            .clone()
            .into_iter()
            .map(|window| (window, None))
            .collect();

        Resources::insert(WindowManager(windows));
    }
}

#[derive(Debug, Clone)]
pub struct Window {
    pub title: String,
}

impl Window {
    #[inline]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: Default::default(),
        }
    }
}
