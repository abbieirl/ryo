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

                #[inline(never)]
                fn sum(self) -> Self::Output {
                    let (prefix, middle, suffix) = self.0.as_simd();

                    let mut sums = Simd::from_array([Default::default(); simd_lanes::<$t>()]);
                    sums[0] = prefix.iter().copied().sum();
                    sums[1] = suffix.iter().copied().sum();

                    middle.iter().copied().fold(sums, Simd::add).reduce_sum()
                }
            }
        )*
    };
}

impl_sum!(f32, f64);
impl_sum!(u8, u16, u32, u64);
impl_sum!(i8, i16, i32, i64);

// TODO: check and impl all simd extensions
pub const fn simd_lanes<T: SimdElement>() -> usize {
    if cfg!(target_cpu = "") {
        avx512_lanes::<T>()
    } else if cfg!(target_feature = "avx") {
        avx_lanes::<T>()
    } else {
        sse_lanes::<T>()
    }
}

const fn sse_lanes<T: SimdElement>() -> usize {
    min_const(128 / size_of::<T>(), 64)
}

const fn avx_lanes<T: SimdElement>() -> usize {
    min_const(256 / size_of::<T>(), 64)
}

const fn avx512_lanes<T: SimdElement>() -> usize {
    min_const(512 / size_of::<T>(), 64)
}

const fn min_const(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}
