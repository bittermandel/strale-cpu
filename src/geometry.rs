use std::sync::Arc;

use crate::{
    material::Material,
    ray::{HitRecord, Ray},
    vec3::Vec3,
};

pub enum Geometry {
    Sphere(Vec3, f32, Arc<dyn Material>),
    MovingSphere {
        center0: Vec3,
        center1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Arc<dyn Material>,
    },
}

impl Geometry {
    pub fn hit(&self, ray: &Ray, closest_hit_distance: f32) -> Option<(HitRecord, f32)> {
        match self {
            Geometry::Sphere(position, radius, material) => {
                let oc = ray.origin - *position;

                let a = ray.direction.length_squared();
                let half_b = oc.dot(ray.direction);
                let c = oc.length_squared() - radius * radius;
                let discriminant = half_b * half_b - a * c;

                if discriminant < 0.0 {
                    return None;
                }

                let sqrtd = discriminant.sqrt();

                let mut root = (-half_b - sqrtd) / a;
                if root < 0.001 || root > closest_hit_distance {
                    root = (-half_b + sqrtd) / a;
                    if root < 0.001 || root > closest_hit_distance {
                        return None;
                    }
                }

                let outward_normal = (ray.at(root) - *position) / *radius;
                let front_face = ray.direction.dot(outward_normal) < 0.0;
                let mut normal: Vec3 = Vec3::new(0.0, 0.0, 0.0);
                if front_face {
                    normal = outward_normal;
                } else {
                    normal = -outward_normal;
                }

                let closest_hit = HitRecord {
                    t: root,
                    p: ray.at(root),
                    normal,
                    front_face,
                    material: material.clone(),
                };

                return Some((closest_hit, root));
            }
            Geometry::MovingSphere {
                center0,
                center1,
                time0,
                time1,
                radius,
                material,
            } => {}
        }
    }
}
