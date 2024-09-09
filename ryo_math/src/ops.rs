pub trait Sum {
    type Output;

    fn sum(self) -> Self::Output;
}