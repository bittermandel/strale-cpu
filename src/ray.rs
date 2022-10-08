use std::sync::Arc;

use glam::Vec3A;

use crate::{aabb::AABB, hittable::Hittable, material::Material, scene::Scene};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3A,
    pub normal: Vec3A,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
    pub time: f32,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3A {
        self.origin + self.direction * t
    }

    pub fn hit(&self, objects: Vec<&Box<dyn Hittable>>) -> Option<HitRecord> {
        let mut closest_hit_distance: f32 = f32::MAX;
        let mut closest_hit: Option<HitRecord> = None;

        for object in objects.iter() {
            match object.hit(self, 0.001, closest_hit_distance) {
                Some(rec) => {
                    if rec.t < closest_hit_distance {
                        closest_hit_distance = rec.t;
                    }
                    closest_hit = Some(rec);
                }
                None => continue,
            }
        }

        closest_hit
    }

    pub fn aabb_intersect(&self, aabb: AABB) -> bool {
        let inv_d = 1.0 / self.direction;

        let ray_min = (aabb.minimum - self.origin) * inv_d;
        let ray_max = (aabb.maximum - self.origin) * inv_d;

        let tsmaller = ray_min.min(ray_max);
        let tbigger = ray_min.max(ray_max);

        let tmin = tsmaller.x.max(tsmaller.y.max(tsmaller.z));
        let tmax = tbigger.x.min(tbigger.y.min(tbigger.z));

        tmin < tmax
    }
}
