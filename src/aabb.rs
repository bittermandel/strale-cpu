use crate::{ray::Ray, vec3::Vec3};

struct AABB {
    minimum: Vec3,
    maximum: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }

    pub fn hit(self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let t0 = ((self.minimum.e[a] - r.origin.e[a]) / r.direction.e[a])
                .min((self.maximum.e[a] - r.origin.e[a]) / r.direction.e[a]);
            let t1 = ((self.minimum.e[a] - r.origin.e[a]) / r.direction.e[a])
                .max((self.maximum.e[a] - r.origin.e[a]) / r.direction.e[a]);

            //t_min = t0.max(t_min);
        }

        true
    }
}
