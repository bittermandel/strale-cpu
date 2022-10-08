use crate::{
    aabb::AABB,
    ray::{HitRecord, Ray},
};

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
}
