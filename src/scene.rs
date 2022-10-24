use std::{fs::File, io::BufReader, sync::Arc};

use glam::{Vec3, Vec3A};
use rand::{distributions::Alphanumeric, rngs::SmallRng, thread_rng, Rng, SeedableRng};
use rand_seeder::Seeder;
use tobj::{LoadOptions, GPU_LOAD_OPTIONS, OFFLINE_RENDERING_LOAD_OPTIONS};

use crate::{
    aabb::AABB,
    geometry::{Sphere, Triangle},
    hittable::Hittable,
    material::{Dialectric, Lambertian, Metal},
    texture::{
        color::{NoiseTexture, SolidColor},
        image::ImageTexture,
    },
};

pub fn get_seed(length: usize) -> String {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(length)
        .collect()
}

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
    pub fn bounding_box(&self, _time0: f32, _time1: f32) -> AABB {
        if self.objects.is_empty() {
            return AABB::empty();
        }

        let mut output_box = AABB::empty();
        let mut first_box = true;

        for object in &self.objects {
            let temp_box = object.bounding_box();

            if first_box {
                output_box = temp_box;
            }
            first_box = false;
        }

        output_box
    }

    pub fn from_obj(path: String) -> Self {
        let (models, materials) =
            tobj::load_obj(path, &LoadOptions::default()).expect("Failed to load obj file");

        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

        let ground_material = Arc::new(Lambertian {
            albedo: Box::new(NoiseTexture::new()),
        });
        objects.push(Box::new(Sphere {
            position: Vec3A::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: ground_material,
        }));

        let mut totsize: usize = 0;

        //let materials = materials.expect("Failed to load MTL file");

        for model in models {
            let mesh = model.mesh;

            // let material = &materials[mesh.material_id.unwrap()];

            // let diffuse = material.diffuse;

            totsize += mesh.indices.len() / 3;

            let positions: Vec<Vec3A> = mesh
                .positions
                .chunks(3)
                .map(|i| Vec3A::new(i[0], i[1], i[2]))
                .collect();

            mesh.indices
                .chunks(3)
                .map(|i| {
                    objects.push(Box::new(Triangle {
                        material: Arc::new(Lambertian {
                            albedo: Box::new(SolidColor::new(1.0, 0.0, 0.0)),
                        }),
                        vertex0: positions[i[0] as usize],
                        vertex1: positions[i[1] as usize],
                        vertex2: positions[i[2] as usize],
                    }))
                })
                .for_each(drop);
        }

        println!("{} triangles", totsize);

        Self { objects }
    }

    pub fn randomize(&mut self) -> &mut Self {
        let ground_material = Arc::new(Lambertian {
            albedo: Box::new(NoiseTexture::new()),
        });

        let mut objects: Vec<Box<dyn Hittable>> = vec![];

        objects.push(Box::new(Sphere {
            position: Vec3A::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: ground_material,
        }));

        let seed = "D4en7gYSdsaaOzPd58BfTa79ugWvcEm5"; //get_seed(32);

        //let mut rng: SmallRng = Seeder::from(seed.clone()).make_rng();
        let mut rng: SmallRng = Seeder::from(seed).make_rng();
        println!("{}", seed);

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat: f32 = rng.gen::<f32>();

                let center = Vec3A::new(
                    a as f32 + 0.9 * rng.gen::<f32>(),
                    0.2,
                    b as f32 + 0.9 * rng.gen::<f32>(),
                );

                if (center - Vec3A::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Vec3A::new(rng.gen(), rng.gen(), rng.gen())
                            * Vec3A::new(rng.gen(), rng.gen(), rng.gen());
                        let texture = Box::new(SolidColor::new(albedo.x, albedo.y, albedo.z));
                        objects.push(Box::new(Triangle {
                            material: Arc::new(Lambertian { albedo: texture }),
                            vertex0: center + Vec3A::new(-0.2, 0.0, 0.2),
                            vertex1: center + Vec3A::new(0.0, 0.0, 0.0),
                            vertex2: center + Vec3A::new(0.0, 0.2, 0.2),
                        }));
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Vec3A::new(
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
            position: Vec3A::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new(Lambertian {
                albedo: Box::new(ImageTexture::new("earthmap.jpg".into())),
            }),
            //material: Arc::new(Dialectric { ir: 1.5 }),
        }));
        objects.push(Box::new(Sphere {
            position: Vec3A::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new(Lambertian {
                albedo: Box::new(SolidColor::new(0.4, 0.2, 0.1)),
            }),
        }));
        objects.push(Box::new(Sphere {
            position: Vec3A::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new(Metal {
                albedo: Vec3A::new(0.7, 0.6, 0.5),
                fuzz: 0.0,
            }),
        }));

        self.objects = objects;

        self
    }

    pub fn plane() -> Vec<Box<dyn Hittable>> {
        let (models, materials) =
            tobj::load_obj("plane.obj", &LoadOptions::default()).expect("Failed to load obj file");

        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

        //let materials = materials.expect("Failed to load MTL file");

        for model in models {
            let mesh = model.mesh;

            let positions: Vec<Vec3A> = mesh
                .positions
                .chunks(3)
                .map(|i| Vec3A::new(i[0], i[1], i[2]) + Vec3A::new(0.9, 0.2, 0.9))
                .collect();

            mesh.indices
                .chunks(3)
                .map(|i| {
                    objects.push(Box::new(Triangle {
                        material: Arc::new(Lambertian {
                            albedo: Box::new(SolidColor::new(1.0, 0.0, 0.0)),
                        }),
                        vertex0: positions[i[0] as usize] * 1000.0,
                        vertex1: positions[i[1] as usize] * 1000.0,
                        vertex2: positions[i[2] as usize] * 1000.0,
                    }))
                })
                .for_each(drop);
        }

        objects
    }

    pub fn randomize_bunnies(&mut self) -> &mut Self {
        let ground_material = Arc::new(Lambertian {
            albedo: Box::new(NoiseTexture::new()),
        });

        let mut objects: Vec<Box<dyn Hittable>> = vec![];

        objects.push(Box::new(Sphere {
            position: Vec3A::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: ground_material,
        }));

        let (models, materials) =
            tobj::load_obj("bunny.obj", &LoadOptions::default()).expect("Failed to load obj file");

        objects.append(&mut Self::plane());

        let materials = materials.expect("Failed to load MTL file");

        let seed = "D4en7gYSdsaaOzPd58BfTa79ugWvcEm5"; //get_seed(32);

        //let mut rng: SmallRng = Seeder::from(seed.clone()).make_rng();
        let mut rng: SmallRng = Seeder::from(seed).make_rng();
        println!("{}", seed);

        for a in -2..2 {
            for b in -2..2 {
                let choose_mat: f32 = rng.gen::<f32>();

                let center = Vec3A::new(
                    a as f32 + 0.9 * rng.gen::<f32>(),
                    0.2,
                    b as f32 + 0.9 * rng.gen::<f32>(),
                );

                if (center - Vec3A::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Vec3A::new(rng.gen(), rng.gen(), rng.gen())
                            * Vec3A::new(rng.gen(), rng.gen(), rng.gen());
                        for model in &models {
                            let mesh = &model.mesh;
                            let material = &materials[mesh.material_id.unwrap()];

                            let texture = &material.diffuse_texture;

                            let bunny_material = Arc::new(Lambertian {
                                albedo: Box::new(ImageTexture::new(texture.to_string())),
                            });

                            let positions: Vec<Vec3A> = mesh
                                .positions
                                .chunks(3)
                                .map(|i| Vec3A::new(i[0], i[1], i[2]))
                                .collect();

                            mesh.indices
                                .chunks(3)
                                .map(|i| {
                                    objects.push(Box::new(Triangle {
                                        material: bunny_material.clone(),
                                        vertex0: positions[i[0] as usize] * 0.005,
                                        vertex1: positions[i[1] as usize] * 0.005,
                                        vertex2: positions[i[2] as usize] * 0.005,
                                    }))
                                })
                                .for_each(drop);
                        }
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Vec3A::new(
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
            position: Vec3A::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new(Dialectric { ir: 1.5 }),
        }));
        objects.push(Box::new(Sphere {
            position: Vec3A::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new(Lambertian {
                albedo: Box::new(SolidColor::new(0.4, 0.2, 0.1)),
            }),
        }));
        objects.push(Box::new(Sphere {
            position: Vec3A::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new(Metal {
                albedo: Vec3A::new(0.7, 0.6, 0.5),
                fuzz: 0.0,
            }),
        }));

        self.objects = objects;

        self
    }
}
