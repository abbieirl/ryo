use ryo::engine::Engine;
use ryo::window::{Window, WindowModule};
use ryo::winit::WinitModule;

fn main() {
    let window_pri = Window::default().with_title("Ryo Pri");
    let window_sec = Window::default().with_title("Ryo Sec");
    
    let window_module = WindowModule::default()
        .with_window(window_pri)
        .with_window(window_sec);

    Engine::default()
        .add_module(window_module)
        .add_module(WinitModule)
        .run();
}
