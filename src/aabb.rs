use glam::Vec3A;

use crate::ray::Ray;

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub minimum: Vec3A,
    pub maximum: Vec3A,
}
#[warn(clippy::upper_case_acronyms)]

impl AABB {
    pub fn new(a: Vec3A, b: Vec3A) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }

    pub fn empty() -> Self {
        Self {
            minimum: Vec3A::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            maximum: Vec3A::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        }
    }

    pub fn hit(&self, r: &Ray, _t_min: f32, _t_max: f32) -> bool {
        self.hit_faster(r)
    }

    pub fn hit_fast(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let inv_d = 1.0 / r.direction;

        let ray_min = (self.minimum - r.origin) * inv_d;
        let ray_max = (self.maximum - r.origin) * inv_d;

        let tsmaller = ray_min.min(ray_max);
        let tbigger = ray_min.max(ray_max);

        let tmin = t_min.max(tsmaller.x.max(tsmaller.y.max(tsmaller.z)));
        let tmax = t_max.min(tbigger.x.min(tbigger.y.min(tbigger.z)));

        tmin < tmax
    }

    pub fn hit_faster(&self, r: &Ray) -> bool {
        let inv_d = 1.0 / r.direction;

        let ray_min = (self.minimum - r.origin) * inv_d;
        let ray_max = (self.maximum - r.origin) * inv_d;

        let tsmaller = ray_min.min(ray_max);
        let tbigger = ray_min.max(ray_max);

        let tmin = tsmaller.x.max(tsmaller.y.max(tsmaller.z));
        let tmax = tbigger.x.min(tbigger.y.min(tbigger.z));

        tmin < tmax
    }
    #[warn(clippy::upper_case_acronyms)]

    pub fn join(box0: &AABB, box1: &AABB) -> AABB {
        let small = Vec3A::new(
            box0.minimum.x.min(box1.minimum.x),
            box0.minimum.y.min(box1.minimum.y),
            box0.minimum.z.min(box1.minimum.z),
        );
        let big = Vec3A::new(
            box0.maximum.x.max(box1.maximum.x),
            box0.maximum.y.max(box1.maximum.y),
            box0.maximum.z.max(box1.maximum.z),
        );

        AABB {
            minimum: small,
            maximum: big,
        }
    }

    pub fn join_mut(&mut self, other: &AABB) {
        let small = Vec3A::new(
            self.minimum.x.min(other.minimum.x),
            self.minimum.y.min(other.minimum.y),
            self.minimum.z.min(other.minimum.z),
        );
        let big = Vec3A::new(
            self.maximum.x.max(other.maximum.x),
            self.maximum.y.max(other.maximum.y),
            self.maximum.z.max(other.maximum.z),
        );

        self.minimum = small;
        self.maximum = big;
    }

    pub fn surface_area(&self) -> f32 {
        let size = self.maximum - self.minimum;
        2.0 * (size.x * size.y + size.x * size.z + size.y * size.z)
    }
}
