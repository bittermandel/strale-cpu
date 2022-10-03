use glam::Vec3;

use crate::ray::Ray;

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy)]
pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3,
}
#[warn(clippy::upper_case_acronyms)]

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        self.hit_fast(r, t_min, t_max)
    }

    pub fn hit_fast(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let inv_d = 1.0 / r.direction;

        let t0s = (self.minimum - r.origin) * inv_d;
        let t1s = (self.maximum - r.origin) * inv_d;

        let tsmaller = t0s.min(t1s);
        let tbigger = t0s.max(t1s);

        let tmin = t_min.max(tsmaller.x.max(tsmaller.y.max(tsmaller.z)));
        let tmax = t_max.min(tbigger.x.min(tbigger.y.min(tbigger.z)));

        tmin < tmax
    }
    #[warn(clippy::upper_case_acronyms)]

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Vec3::new(
            box0.minimum.x.min(box1.minimum.x),
            box0.minimum.y.min(box1.minimum.y),
            box0.minimum.z.min(box1.minimum.z),
        );
        let big = Vec3::new(
            box0.maximum.x.max(box1.maximum.x),
            box0.maximum.y.max(box1.maximum.y),
            box0.maximum.z.max(box1.maximum.z),
        );

        AABB {
            minimum: small,
            maximum: big,
        }
    }
}
