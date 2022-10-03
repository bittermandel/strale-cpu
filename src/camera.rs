use glam::Vec3A;
use rand::{thread_rng, Rng};

use crate::{
    ray::Ray,
    util::degrees_to_radians,
    vec3::{random_in_unit_disk, unit_vector},
};

pub struct Camera {
    pub origin: Vec3A,
    pub lower_left_corner: Vec3A,
    pub horizontal: Vec3A,
    pub vertical: Vec3A,
    pub u: Vec3A,
    pub v: Vec3A,
    pub w: Vec3A,
    pub lens_radius: f32,
    pub _time0: f32,
    pub _time1: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3A,
        lookat: Vec3A,
        vup: Vec3A,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        // Camera
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(vup.cross(w));
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            w,
            u,
            v,
            lens_radius,
            _time0: 0.0,
            _time1: 1.0,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        let mut rng = thread_rng();

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
            time: rng.gen(),
        }
    }
}
