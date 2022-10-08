use glam::Vec3A;

use crate::{axis::Axis, ray::Ray};

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

    pub fn size(&self) -> Vec3A {
        self.maximum - self.minimum
    }

    pub fn center(&self) -> Vec3A {
        self.minimum + (self.size() / 2.0)
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

    pub fn join(self: &AABB, box1: &AABB) -> AABB {
        let small = Vec3A::new(
            self.minimum.x.min(box1.minimum.x),
            self.minimum.y.min(box1.minimum.y),
            self.minimum.z.min(box1.minimum.z),
        );
        let big = Vec3A::new(
            self.maximum.x.max(box1.maximum.x),
            self.maximum.y.max(box1.maximum.y),
            self.maximum.z.max(box1.maximum.z),
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

    pub fn grow(&self, other: &Vec3A) -> AABB {
        AABB::new(
            Vec3A::new(
                self.minimum.x.min(other.x),
                self.minimum.y.min(other.y),
                self.minimum.z.min(other.z),
            ),
            Vec3A::new(
                self.maximum.x.max(other.x),
                self.maximum.y.max(other.y),
                self.maximum.z.max(other.z),
            ),
        )
    }

    pub fn grow_mut(&mut self, other: &Vec3A) {
        self.minimum = Vec3A::new(
            self.minimum.x.min(other.x),
            self.minimum.y.min(other.y),
            self.minimum.z.min(other.z),
        );
        self.maximum = Vec3A::new(
            self.maximum.x.max(other.x),
            self.maximum.y.max(other.y),
            self.maximum.z.max(other.z),
        );
    }

    pub fn surface_area(&self) -> f32 {
        2.0 * (self.size().x * self.size().y
            + self.size().x * self.size().z
            + self.size().y * self.size().z)
    }

    pub fn largest_axis(&self) -> Axis {
        let size = self.size();
        if size.x > size.y && size.x > size.z {
            Axis::X
        } else if size.y > size.z {
            Axis::Y
        } else {
            Axis::Z
        }
    }
}
