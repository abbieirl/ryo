use ryo::engine::Engine;
use ryo::window::WindowModule;
use ryo::winit::WinitModule;

fn main() {
    Engine::default()
        .add_module(WindowModule)
        .add_module(WinitModule)
        .run();
}
