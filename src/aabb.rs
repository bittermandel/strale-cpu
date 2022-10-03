use std::{mem::swap, time::Instant};

use crate::{ray::Ray, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }

    pub fn hit(&self, r: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        return self.hit_fast(r, t_min, t_max);

        /*for a in 0..3 {
            let invD = 1.0 / r.direction.e[a];
            let mut t0 = (self.minimum.e[a] - r.origin.e[a]) * invD;
            let mut t1 = (self.maximum.e[a] - r.origin.e[a]) * invD;

            if invD < 0.0 {
                swap(&mut t0, &mut t1);
            }

            if t1.min(t_max) <= t0.max(t_min) {
                return false;
            }
        }

        true*/
    }

    pub fn hit_fast(&self, r: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        let invD = 1.0 / r.direction;

        let t0s = (self.minimum - r.origin) * invD;
        let t1s = (self.maximum - r.origin) * invD;

        let tsmaller = t0s.min(t1s);
        let tbigger = t0s.max(t1s);

        let tmin = t_min.max(tsmaller.x().max(tsmaller.y().max(tsmaller.z())));
        let tmax = t_max.min(tbigger.x().min(tbigger.y().min(tbigger.z())));

        return tmin < tmax;
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Vec3::new(
            box0.minimum.x().min(box1.minimum.x()),
            box0.minimum.y().min(box1.minimum.y()),
            box0.minimum.z().min(box1.minimum.z()),
        );
        let big = Vec3::new(
            box0.maximum.x().max(box1.maximum.x()),
            box0.maximum.y().max(box1.maximum.y()),
            box0.maximum.z().max(box1.maximum.z()),
        );

        return AABB {
            minimum: small,
            maximum: big,
        };
    }
}
