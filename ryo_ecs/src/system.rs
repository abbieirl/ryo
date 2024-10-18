use core::any::type_name_of_val;
use core::fmt::{Debug, Formatter, Result};

pub trait System: 'static {
    fn run(&self);
}

impl<T> System for T
where
    T: Fn() + 'static + Send + Sync,
{
    fn run(&self) {
        self()
    }
}

impl Debug for dyn System {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(type_name_of_val(self))
    }
}
