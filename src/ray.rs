use std::sync::Arc;

use crate::{material::Material, scene::Scene, vec3::Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn hit(&self, scene: &Scene) -> Option<HitRecord> {
        let mut closest_hit_distance: f32 = f32::MAX;
        let mut closest_hit: Option<HitRecord> = None;

        for object in scene.objects.iter() {
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
}
