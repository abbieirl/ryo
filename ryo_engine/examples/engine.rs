use ryo_engine::engine::Engine;

fn main() {
    Engine::default().set_runner(|_engine| {}).run();
}
