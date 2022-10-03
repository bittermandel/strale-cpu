use std::ops::*;

use glam::Vec3;
use rand::{thread_rng, Rng};

pub fn near_zero(a: Vec3) -> bool {
    let s = 1e-8;
    a[0].abs() < s && a[1].abs() < s && a[1].abs() < s
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

#[allow(dead_code)]
pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = n.dot(-uv).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = thread_rng();

    loop {
        let p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        };
        return p;
    }
}
