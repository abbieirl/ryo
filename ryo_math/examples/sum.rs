use ryo_math::{ops::Sum, vector::Vector};

fn main() {
    let vec = Vector::from([1.0, 2.0, 3.0]);
    core::hint::black_box(vec.sum());
}
