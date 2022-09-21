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
        return self.origin + self.direction * t;
    }

    pub fn hit(&self, scene: &Scene) -> Option<HitRecord> {
        let mut closest_hit_distance: f32 = f32::MAX;
        let mut closest_hit: Option<HitRecord> = None;

        let objects = scene.objects();

        for object in objects.iter() {
            match object.hit(&self, closest_hit_distance) {
                Some((rec, distance)) => {
                    closest_hit = Some(rec);
                    if distance < closest_hit_distance {
                        closest_hit_distance = distance;
                    }
                }
                None => continue,
            }
        }

        return closest_hit;
    }
}
