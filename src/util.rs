use std::{cmp::Ordering, f32::consts::PI};

use glam::Vec3;

use crate::{aabb::AABB, hittable::Hittable};

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn joint_aabb(
    indices: &[usize],
    shapes: &Vec<Box<dyn Hittable>>,
    time0: f32,
    time1: f32,
) -> AABB {
    let mut aabb = AABB::empty();

    for index in indices {
        aabb.join_mut(&shapes[*index].bounding_box(time0, time1).unwrap());
    }

    return aabb;
}

pub fn joint_aabb_from_shapes(shapes: &Vec<Box<dyn Hittable>>, time0: f32, time1: f32) -> AABB {
    let mut aabb = AABB::empty();

    for shape in shapes {
        aabb.join_mut(&shape.bounding_box(time0, time1).unwrap());
    }

    return aabb;
}
