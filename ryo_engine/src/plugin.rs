use crate::engine::Engine;

pub trait Plugin {
    fn build(&self, engine: &mut Engine);
}