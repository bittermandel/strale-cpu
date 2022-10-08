use std::f32::consts::PI;

use crate::{aabb::AABB, hittable::Hittable};

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn joint_aabb(indices: &[usize], shapes: &[Box<dyn Hittable>]) -> AABB {
    let mut aabb = AABB::empty();

    for index in indices {
        aabb.join_mut(&shapes[*index].bounding_box());
    }

    aabb
}

pub fn joint_aabb_from_shapes(shapes: &[Box<dyn Hittable>]) -> AABB {
    let mut aabb = AABB::empty();

    for shape in shapes {
        aabb.join_mut(&shape.bounding_box());
    }

    aabb
}

pub fn concatenate_vectors<T: Sized>(vectors: &mut [Vec<T>]) -> Vec<T> {
    let mut vec = Vec::new();

    for vector in vectors.iter_mut() {
        vec.append(vector);
    }

    vec
}
