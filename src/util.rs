use std::{cmp::Ordering, f32::consts::PI};

use glam::Vec3;

use crate::{aabb::AABB, hittable::Hittable};

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn joint_aabb(indices: &[usize], shapes: &Vec<Box<dyn Hittable>>) -> AABB {
    let mut aabb = AABB::empty();

    for index in indices {
        aabb.join_mut(&shapes[*index].bounding_box().unwrap());
    }

    return aabb;
}

pub fn joint_aabb_from_shapes(shapes: &Vec<Box<dyn Hittable>>) -> AABB {
    let mut aabb = AABB::empty();

    for shape in shapes {
        aabb.join_mut(&shape.bounding_box().unwrap());
    }

    return aabb;
}
