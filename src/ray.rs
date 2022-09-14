use crate::{geometry::Geometry, scene::Scene, vec3::Vec3};

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        return self.origin + self.direction * t;
    }

    pub fn hit(&self, scene: &Scene) -> Option<HitRecord> {
        let mut closest_hit_distance: f32 = f32::MAX;
        let mut closest_hit: Option<HitRecord> = None;

        let objects = &scene.objects();

        for object in objects.iter() {
            match object {
                Geometry::Sphere(position, radius) => {
                    let oc = self.origin - *position;

                    let a = self.direction.length_squared();
                    let half_b = oc.dot(self.direction);
                    let c = oc.length_squared() - radius * radius;
                    let discriminant = half_b * half_b - a * c;

                    if discriminant < 0.0 {
                        continue;
                    }

                    let sqrtd = discriminant.sqrt();

                    let mut root = (-half_b - sqrtd) / a;
                    if root < 0.001 || root > closest_hit_distance {
                        root = (-half_b + sqrtd) / a;
                        if root < 0.001 || root > closest_hit_distance {
                            continue;
                        }
                    }

                    closest_hit_distance = root;

                    let outward_normal = (self.at(root) - *position) / *radius;
                    let front_face = self.direction.dot(outward_normal) < 0.0;
                    let mut normal: Vec3 = Vec3::new(0.0, 0.0, 0.0);
                    if front_face {
                        normal = outward_normal;
                    } else {
                        normal = -outward_normal;
                    }

                    closest_hit = Some(HitRecord {
                        t: root,
                        p: self.at(root),
                        normal: normal,
                        front_face: front_face,
                    });
                }
            }
        }

        return closest_hit;
    }
}
