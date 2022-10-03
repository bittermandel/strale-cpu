use std::sync::Arc;

use glam::Vec3;
use rand::{thread_rng, Rng};

use crate::{
    aabb::AABB,
    bvh::Bvh,
    geometry::Sphere,
    hittable::Hittable,
    material::{Dialectric, Lambertian, Metal},
    texture::{CheckerTexture, SolidColor},
};

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    #[allow(dead_code)]
    pub fn new() -> Scene {
        Scene { objects: vec![] }
    }

    #[allow(dead_code)]
    pub fn set(&mut self, objects: Vec<Box<dyn Hittable>>) {
        self.objects = objects;
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    #[allow(dead_code)]
    pub fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output_box: Option<AABB> = None;
        let mut first_box = true;

        for object in &self.objects {
            let temp_box = object.bounding_box(time0, time1);

            temp_box?;

            if first_box {
                output_box = temp_box;
            }
            first_box = false;
        }

        output_box
    }

    pub fn randomize(&mut self) -> &mut Self {
        let ground_material = Arc::new(Lambertian {
            albedo: Box::new(CheckerTexture::new_from_colors(
                Vec3::new(0.2, 0.3, 0.1),
                Vec3::new(0.9, 0.9, 0.9),
            )),
        });

        let mut objects: Vec<Box<dyn Hittable>> = vec![];

        objects.push(Box::new(Sphere {
            position: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: ground_material,
        }));

        let mut rng = thread_rng();

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat: f32 = rng.gen::<f32>();

                let center = Vec3::new(
                    a as f32 + 0.9 * rng.gen::<f32>(),
                    0.2,
                    b as f32 + 0.9 * rng.gen::<f32>(),
                );

                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Vec3::new(rng.gen(), rng.gen(), rng.gen())
                            * Vec3::new(rng.gen(), rng.gen(), rng.gen());
                        let texture = Box::new(SolidColor::new(albedo.x, albedo.y, albedo.z));
                        objects.push(Box::new(Sphere {
                            position: center,
                            radius: 0.2,
                            material: Arc::new(Lambertian { albedo: texture }),
                        }));
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Vec3::new(
                            rng.gen_range(0.5..1.0),
                            rng.gen_range(0.5..1.0),
                            rng.gen_range(0.5..1.0),
                        );
                        let fuzz: f32 = rng.gen_range(0.0..0.5);
                        objects.push(Box::new(Sphere {
                            position: center,
                            radius: 0.2,
                            material: Arc::new(Metal { albedo, fuzz }),
                        }));
                    } else {
                        // glass
                        objects.push(Box::new(Sphere {
                            position: center,
                            radius: 0.2,
                            material: Arc::new(Dialectric { ir: 1.5 }),
                        }));
                    }
                }
            }
        }

        objects.push(Box::new(Sphere {
            position: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new(Dialectric { ir: 1.5 }),
        }));
        objects.push(Box::new(Sphere {
            position: Vec3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new(Lambertian {
                albedo: Box::new(SolidColor::new(0.4, 0.2, 0.1)),
            }),
        }));
        objects.push(Box::new(Sphere {
            position: Vec3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new(Metal {
                albedo: Vec3::new(0.7, 0.6, 0.5),
                fuzz: 0.0,
            }),
        }));

        self.objects.push(Box::new(Bvh::new(objects, 0.0, 1.0)));

        self
    }
}
