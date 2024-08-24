use sealed::Sealed;

mod sealed {
    pub trait Sealed {}
}

pub trait Number: Default + Sealed {
    /// Largest finite `Number` value.
    const MAX: Self;

    /// Smallest finite `Number` value.
    const MIN: Self;
}

impl Sealed for f32 {}

impl Number for f32 {
    const MAX: Self = Self::MAX;

    const MIN: Self = Self::MIN;
}

pub trait Float: Number {}