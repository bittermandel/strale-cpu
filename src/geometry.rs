use std::{f32::consts::PI, sync::Arc};

use glam::Vec3A;

use crate::{
    aabb::AABB,
    hittable::Hittable,
    material::Material,
    ray::{HitRecord, Ray},
};

pub struct Sphere {
    pub position: Vec3A,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn get_sphere_uv(p: Vec3A) -> (f32, f32) {
        let theta = -p.y.acos();
        let phi = -p.z.atan2(p.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.position;

        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let outward_normal = (ray.at(root) - self.position) / self.radius;
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let mut normal = outward_normal;
        if !front_face {
            normal = -outward_normal;
        }

        let uv_coords = Sphere::get_sphere_uv(outward_normal);

        let closest_hit = HitRecord {
            t: root,
            p: ray.at(root),
            normal,
            u: uv_coords.0,
            v: uv_coords.1,
            front_face,
            material: self.material.clone(),
        };

        Some(closest_hit)
    }
    fn bounding_box(&self) -> AABB {
        AABB {
            minimum: self.position - Vec3A::new(self.radius, self.radius, self.radius),
            maximum: self.position + Vec3A::new(self.radius, self.radius, self.radius),
        }
    }
}
