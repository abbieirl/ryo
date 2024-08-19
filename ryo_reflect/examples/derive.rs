use ryo_reflect::prelude::*;

#[derive(Default, Reflect)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    let vec3 = Vec3 {
        x: 55.0,
        y: 1.0,
        z: 0.0,
    };

    let reflect = vec3.as_struct();
    dbg!(reflect);
}
