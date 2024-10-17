use crate::engine::Engine;

pub trait Module {
    fn build(&self, engine: &mut Engine);
}
