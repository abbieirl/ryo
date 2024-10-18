use ryo_engine::{Engine, Module};

pub struct WgpuModule;

impl Module for WgpuModule {
    fn build(&self, _engine: &mut Engine) {}
}
