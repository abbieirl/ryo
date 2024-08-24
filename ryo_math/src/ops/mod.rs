pub trait Dot<Rhs = Self> {
    type Output;

    #[must_use]
    fn dot(self, rhs: Rhs) -> Self::Output;
}

pub trait Distance<Rhs = Self> {
    type Output;

    #[must_use]
    fn distance(self, rhs: Rhs) -> Self::Output;
}
