use core::any::type_name_of_val;
use core::fmt::{Debug, Formatter, Result};

pub trait Component: Send + Sync + 'static {}

impl Debug for dyn Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(type_name_of_val(self))
    }
}
