use core::array::from_fn;
use core::ops::Add;
use num_traits::Num;

pub type Vector4<T> = Vector<T, 4>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector2<T> = Vector<T, 2>;

#[derive(Debug, PartialEq, Eq)]
pub struct Vector<T: Num, const N: usize>([T; N]);

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Num + Copy,
{
    fn default() -> Self {
        Self([T::zero(); N])
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Num + Copy,
{
    pub fn dot(&self, rhs: Vector<T, N>) -> T {
        self.0
            .iter()
            .zip(rhs.0.iter())
            .map(|(&lhs, &rhs)| lhs.mul(rhs))
            .fold(T::zero(), |acc, val| acc.add(val))
    }
}

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Num + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(from_fn(|index| self.0[index].add(rhs.0[index])))
    }
}

impl<T: Num, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(value: [T; N]) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test {
    use super::Vector;

    #[test]
    fn test_default() {
        let lhs = Vector::<i32, 3>::default();
        let rhs = Vector::<i32, 3>::from([0, 0, 0]);

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_dot() {
        let lhs = Vector::from([1, 2, 3]);
        let rhs = Vector::from([4, 5, 6]);

        assert_eq!(lhs.dot(rhs), 32);
    }

    #[test]
    fn test_add() {
        let lhs = Vector::from([1, 2, 3]);
        let rhs = Vector::from([4, 5, 6]);

        assert_eq!(lhs + rhs, Vector::from([5, 7, 9]));
    }
}
