use ryo_math::vector::Vector;

fn main() {
    let _sum = Vector::from([
        1.0, 1.0, 1.0, 2.0, 1.0, 55.0, 1.0, 1.0, 1.0, 2.0, 1.0, 55.0, 1.0, 1.0, 1.0, 2.0, 1.0, 55.0,
    ]);
    
    println!("{}", _sum.sum());
}
