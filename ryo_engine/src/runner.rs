use crate::engine::Engine;
use core::any::type_name_of_val;
use core::fmt::{Debug, Formatter, Result};

pub trait Runner: FnOnce(Engine) + 'static {
    fn run(self: Box<Self>, engine: Engine);
}

impl<T> Runner for T
where
    T: FnOnce(Engine) + 'static,
{
    #[inline]
    fn run(self: Box<Self>, engine: Engine) {
        (self)(engine)
    }
}

impl Default for Box<dyn Runner> {
    fn default() -> Self {
        Box::new(|_engine| {})
    }
}

impl Debug for dyn Runner {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(type_name_of_val(self))
    }
}
