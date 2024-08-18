use ryo_reflect::{r#struct::Struct, reflect::Reflect};

#[derive(Default)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Struct for Vec3 {
    fn field(&self, name: &str) -> Option<&dyn Reflect> {
        match name {
            "x" => Some(&self.x),
            "y" => Some(&self.y),
            "z" => Some(&self.z),
            _ => None,
        }
    }

    fn field_mut(&mut self, name: &str) -> Option<&mut dyn Reflect> {
        match name {
            "x" => Some(&mut self.x),
            "y" => Some(&mut self.y),
            "z" => Some(&mut self.z),
            _ => None,
        }
    }

    fn field_idx_mut(&mut self, idx: usize) -> Option<&mut dyn Reflect> {
        match idx {
            0 => Some(&mut self.x),
            1 => Some(&mut self.y),
            2 => Some(&mut self.z),
            _ => None,
        }
    }

    fn field_count(&self) -> usize {
        3
    }
}

fn main() {
    let mut vec3 = Vec3 {
        x: 55.0,
        y: 1.0,
        z: 0.0,
    };

    let field = vec3.field_mut("x").unwrap();
    dbg!(field.downcast_mut::<f32>());

    let reflect: &dyn Reflect = &vec3;
    dbg!(reflect);
}
