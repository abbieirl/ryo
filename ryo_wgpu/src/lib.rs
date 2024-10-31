use ryo_engine::{Engine, Module, Resource, Resources};
use ryo_window::WindowManager;
use wgpu::*;

pub struct WgpuModule;

impl Module for WgpuModule {
    fn build(&self, _engine: &mut Engine) {
        let wgpu_state = WgpuState::new().await;

        Resources::insert(wgpu_state);
    }
}

struct WgpuState {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
}

impl Resource for WgpuState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl WgpuState {
    async fn new() -> Self {
        let instance = Instance::default();

        let window_manager = Resources::get::<WindowManager>().unwrap();

        // let surface_target = unsafe {
        //     SurfaceTargetUnsafe::from_window(&window_manager.0[0].1.as_ref().unwrap().clone()).unwrap()
        // };
    
        // let surface = unsafe { instance.create_surface_unsafe(surface_target) }.unwrap();

        let surface = instance.create_surface(window_manager.0[0].1.as_ref().unwrap().clone()).unwrap();

        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default(), None)
            .await
            .unwrap();

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: todo!() ,
            width: todo!(),
            height: todo!(),
            present_mode: todo!(),
            desired_maximum_frame_latency: todo!(),
            alpha_mode: todo!(),
            view_formats: todo!(),
        };

        Self {
            surface,
            device,
            queue,
            config: todo!(),
        }
    }
}
