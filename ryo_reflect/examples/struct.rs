use core::any::Any;
use ryo_reflect::prelude::*;

#[derive(Default)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Reflect for Vec3 {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn as_reflect(&self) -> &dyn Reflect {
        self
    }

    fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        self
    }
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

    fn field_index(&self, index: usize) -> Option<&dyn Reflect> {
        match index {
            0 => Some(&self.x),
            1 => Some(&self.y),
            2 => Some(&self.z),
            _ => None,
        }
    }

    fn field_index_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
        match index {
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
    dbg!(field);

    let reflect: &dyn Reflect = &vec3;
    dbg!(reflect);
}
