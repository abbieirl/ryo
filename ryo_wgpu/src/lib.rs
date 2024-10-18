use ryo_engine::{Engine, Module, Resources};
use ryo_window::WindowManager;
use wgpu::*;

pub struct WgpuModule;

impl Module for WgpuModule {
    fn build(&self, _engine: &mut Engine) {}
}

struct WgpuState<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
}

impl<'a> WgpuState<'a> {
    async fn new() -> Self {
        let instance = Instance::default();

        let window_manager = Resources::get::<WindowManager>().unwrap();

        let surface_target = unsafe {
            SurfaceTargetUnsafe::from_window(&window_manager.0[0].1.clone().unwrap()).unwrap()
        };

        let surface = unsafe { instance.create_surface_unsafe(surface_target) }.unwrap();

        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default(), None)
            .await
            .unwrap();

        todo!()
    }
}
