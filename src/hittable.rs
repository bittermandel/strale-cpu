use crate::{
    aabb::AABB,
    bvh2::BVHNode,
    ray::{HitRecord, Ray},
};

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB>;
}
