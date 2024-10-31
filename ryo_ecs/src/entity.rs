#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    index: u32,
}

impl Entity {
    pub fn index(&self) -> u32 {
        self.index
    }
}