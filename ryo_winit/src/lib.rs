use ryo_engine::{Engine, Module, Resources};
use ryo_window::WindowManager;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Debug)]
pub struct WinitModule;

struct EngineWrapper(Engine);

impl Module for WinitModule {
    fn build(&self, engine: &mut Engine) {
        engine.set_runner(winit_runner);
    }
}

impl ApplicationHandler for EngineWrapper {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        Resources::get_mut::<WindowManager>()
            .unwrap()
            .0
            .iter_mut()
            .for_each(|(window, handle)| {
                *handle = Some(Box::new(
                    event_loop
                        .create_window(Window::default_attributes().with_title(&window.title))
                        .unwrap(),
                ));
            });
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => self.0.systems().iter().for_each(|system| system.run()),
            _ => (),
        }
    }
}

fn winit_runner(engine: Engine) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut EngineWrapper(engine)).unwrap();
}
