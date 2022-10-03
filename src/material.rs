use glam::Vec3A;
use rand::Rng;

use crate::{
    ray::{HitRecord, Ray},
    texture::Texture,
    vec3::{near_zero, random_in_unit_sphere, random_unit_vector, refract, unit_vector},
};

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Result<(Vec3A, Ray), ()>;
}

pub struct Lambertian {
    pub albedo: Box<dyn Texture + Send + Sync>,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Result<(Vec3A, Ray), ()> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }

        let scattered = Ray {
            direction: scatter_direction,
            origin: rec.p,
            time: ray.time,
        };
        Ok((self.albedo.value(rec.u, rec.v, rec.p), scattered))
    }
}

pub struct Metal {
    pub albedo: Vec3A,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Result<(Vec3A, Ray), ()> {
        let uvec = unit_vector(ray.direction);
        let reflected = uvec - (2.0 * uvec.dot(rec.normal)) * rec.normal;

        let scattered = Ray {
            direction: reflected + self.fuzz * random_in_unit_sphere(),
            origin: rec.p,
            time: ray.time,
        };
        if scattered.direction.dot(rec.normal) > 0.0 {
            Ok((self.albedo, scattered))
        } else {
            Err(())
        }
    }
}

pub struct Dialectric {
    pub ir: f32,
}

impl Dialectric {
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Result<(Vec3A, Ray), ()> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(ray.direction);

        let cos_theta = rec.normal.dot(-unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let mut rng = rand::thread_rng();

        let direction =
            if cannot_refract || Dialectric::reflectance(cos_theta, refraction_ratio) > rng.gen() {
                unit_direction - (2.0 * unit_direction.dot(rec.normal)) * rec.normal
            } else {
                refract(unit_direction, rec.normal, refraction_ratio)
            };

        let scattered = Ray {
            direction,
            origin: rec.p,
            time: ray.time,
        };
        {
            Ok((Vec3A::new(1.0, 1.0, 1.0), scattered))
        }
    }
}
