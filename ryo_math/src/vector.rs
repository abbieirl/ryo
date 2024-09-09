use core::iter::Sum;
use core::ops::Add;
use core::simd::{Simd, SimdElement};
use std::intrinsics::simd::simd_reduce_add_ordered;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Vector<T, const D: usize>([T; D])
where
    T: SimdElement,
    [T; D]: Default;

impl<T, const D: usize> Vector<T, D>
where
    T: SimdElement + Default + Sum,
    Simd<T, 4>: Add<Output = Simd<T, 4>>,
    [T; D]: Default,
{
    #[inline(never)]
    pub fn sum(&self) -> T {
        let (prefix, middle, suffix) = self.0.as_simd();

        let sums = Simd::from_array([
            prefix.iter().copied().sum(),
            T::default(),
            T::default(),
            suffix.iter().copied().sum(),
        ]);

        let sums: Simd<T, 4> = middle.iter().copied().fold(sums, Simd::add);

        unsafe { simd_reduce_add_ordered(sums, T::default()) }
    }
}

impl<T, const D: usize> From<[T; D]> for Vector<T, D>
where
    T: SimdElement,
    [T; D]: Default,
{
    #[inline(always)]
    fn from(value: [T; D]) -> Self {
        Self(value)
    }
}
