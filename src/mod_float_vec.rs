use bevy::prelude::*;
use crate::ModFloat;

pub struct ModVec3 {
    pub x: ModFloat,
    pub y: ModFloat,
    pub z: ModFloat,
}

impl ModVec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        ModVec3 {
            x: ModFloat::new(x),
            y: ModFloat::new(y),
            z: ModFloat::new(z),
        }
    }

    pub fn from_vec3(v: &Vec3) -> Self {
        ModVec3 {
            x: ModFloat::new(v.x),
            y: ModFloat::new(v.y),
            z: ModFloat::new(v.z),
        }
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x.0, self.y.0, self.z.0)
    }
}