use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::{
    geometry::Geometry,
    material::{Dialectric, Lambertian, Metal},
    vec3::Vec3,
};

pub struct Scene {
    pub objects: Vec<Geometry>,
}

impl Scene {
    pub fn new(objects: Vec<Geometry>) -> Scene {
        Scene { objects }
    }

    pub fn add(&mut self, object: Geometry) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn objects(&self) -> &Vec<Geometry> {
        return &self.objects;
    }

    pub fn random_scene() -> Scene {
        let mut objects: Vec<Geometry> = vec![];

        let ground_material = Arc::new(Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        });
        objects.push(Geometry::Sphere(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        ));

        let mut rng = thread_rng();

        for a in (-11..11) {
            for b in (-11..11) {
                let choose_mat: f32 = rng.gen::<f32>();

                let center = Vec3::new(
                    a as f32 + 0.9 * rng.gen::<f32>(),
                    0.2,
                    b as f32 + 0.9 * rng.gen::<f32>(),
                );

                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Vec3::random() * Vec3::random();
                        /*let center2 = center + Vec3::new(0.0, rng.gen(), 0.0);
                        objects.push(Geometry::MovingSphere {
                            center0: center,
                            center1: center2,
                            time0: 0.0,
                            time1: 1.0,
                            radius: 0.2,
                            material: Arc::new(Lambertian { albedo }),
                        });*/
                        objects.push(Geometry::Sphere(
                            center,
                            0.2,
                            Arc::new(Lambertian { albedo }),
                        ));
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Vec3::random_from(0.5, 1.0);
                        let fuzz: f32 = rng.gen_range(0.0..0.5);
                        objects.push(Geometry::Sphere(
                            center,
                            0.2,
                            Arc::new(Metal { albedo, fuzz }),
                        ));
                    } else {
                        // glass
                        objects.push(Geometry::Sphere(
                            center,
                            0.2,
                            Arc::new(Dialectric { ir: 1.5 }),
                        ));
                    }
                }
            }
        }

        objects.push(Geometry::Sphere(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Arc::new(Dialectric { ir: 1.5 }),
        ));
        objects.push(Geometry::Sphere(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Arc::new(Lambertian {
                albedo: Vec3::new(0.4, 0.2, 0.1),
            }),
        ));
        objects.push(Geometry::Sphere(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Arc::new(Metal {
                albedo: Vec3::new(0.7, 0.6, 0.5),
                fuzz: 0.0,
            }),
        ));

        Scene { objects }
    }
}
