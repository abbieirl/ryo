use crate::world::World;
use core::any::type_name_of_val;
use core::fmt::{Debug, Formatter, Result};

pub trait System: 'static {
    fn run(&self, world: &World);
}

impl<F> System for F
where
    F: Fn(&World) + 'static + Send + Sync,
{
    #[inline]
    fn run(&self, world: &World) {
        self(world)
    }
}

impl Debug for dyn System {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(type_name_of_val(self))
    }
}
