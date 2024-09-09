use crate::ops::Sum;
use core::ops::Add;
use core::simd::num::{SimdFloat, SimdInt, SimdUint};
use core::simd::{Simd, SimdElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Vector<T, const D: usize>([T; D])
where
    T: SimdElement;

impl<T, const D: usize> Default for Vector<T, D>
where
    T: SimdElement + Default,
{
    #[inline]
    fn default() -> Self {
        Self([T::default(); D])
    }
}

impl<T, const D: usize> From<[T; D]> for Vector<T, D>
where
    T: SimdElement,
{
    #[inline]
    fn from(value: [T; D]) -> Self {
        Self(value)
    }
}

macro_rules! impl_sum {
    ($($t:ty),*) => {
        $(
            impl<const D: usize> Sum for Vector<$t, D> {
                type Output = $t;

                fn sum(self) -> Self::Output {
                    let (prefix, middle, suffix) = self.0.as_simd();

                    // let sums = Simd::from_array([
                    //     prefix.iter().copied().sum(),
                    //     Default::default(),
                    //     Default::default(),
                    //     suffix.iter().copied().sum(),
                    // ]);

                    let mut sums = Simd::from_array([Default::default(); sse_lanes::<$t>()]);
                    sums[0] = prefix.iter().copied().sum();
                    sums[1] = suffix.iter().copied().sum();

                    let sums = middle.iter().copied().fold(sums, Simd::add);
                    sums.reduce_sum()
                }
            }
        )*
    };
}

impl_sum!(f32, f64);
impl_sum!(u8, u16, u32, u64);
impl_sum!(i8, i16, i32, i64);

const fn sse_lanes<T>() -> usize
where
    T: SimdElement,
{
    match core::mem::size_of::<T>() {
        1 => 64, // 128 / 1 = 128, but the largest supported lane count is 64
        2 => 64, // 128 / 2 = 64
        4 => 32, // 128 / 4 = 32
        8 => 16, // 128 / 8 = 16
        _ => 1,  // Fallback
    }
}
