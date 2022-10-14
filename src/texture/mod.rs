use std::fmt::Debug;

use glam::Vec3A;

pub mod color;
pub mod image;

pub trait Texture: Debug {
    fn value(&self, u: f32, v: f32, p: Vec3A) -> Vec3A;
}
