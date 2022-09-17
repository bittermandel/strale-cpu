use rayon::prelude::*;
use std::{fs::File, io::BufWriter, path::Path, sync::Arc, vec};

use camera::Camera;
use geometry::Geometry;
use indicatif::ProgressBar;
use material::{Dialectric, Lambertian, Metal};
use rand::Rng;
use ray::Ray;
use scene::Scene;
use vec3::{unit_vector, Vec3};

mod camera;
mod geometry;
mod material;
mod ray;
mod scene;
mod vec3;

const MAX_DEPTH: u32 = 50;
const SAMPLES_PER_PIXEL: u32 = 50;

fn main() {
    let path = Path::new("image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let aspect_ratio = 16.0 / 9.0;

    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;

    let camera = Camera::new();

    let mut encoder = png::Encoder::new(w, image_width, image_height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::Fast);

    let mut writer = encoder.write_header().unwrap();

    let mut data: Vec<u8> = vec![];

    let bar = ProgressBar::new(image_height as u64);

    let material_ground = Arc::new(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });
    let material_center = Arc::new(Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });
    let material_left = Arc::new(Dialectric { ir: 1.5 });
    let material_right = Arc::new(Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    });

    let scene = Scene::new(vec![
        Geometry::Sphere(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground),
        Geometry::Sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center),
        Geometry::Sphere(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left),
        Geometry::Sphere(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right),
    ]);

    let mut pixelvecs: Vec<Vec<Vec3>> = vec![];

    (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            bar.inc(1);
            let pixels: Vec<Vec3> = (0..image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

                    let mut rng = rand::thread_rng();

                    for _ in 0..SAMPLES_PER_PIXEL {
                        let u = (i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
                        let v = (j as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;

                        let r: Ray = Ray {
                            origin: camera.origin,
                            direction: camera.lower_left_corner
                                + u * camera.horizontal
                                + v * camera.vertical
                                - camera.origin,
                        };

                        let color = ray_color(&scene, &r, MAX_DEPTH);
                        pixel_color += color;
                    }

                    let scale = 1.0 / SAMPLES_PER_PIXEL as f32;

                    pixel_color * scale
                })
                .collect();

            pixels
        })
        .collect_into_vec(&mut pixelvecs);

    for pixelvec in pixelvecs {
        for pixel in pixelvec {
            data.push((255.99 * (pixel.x()).sqrt().clamp(0.0, 0.999)) as u8);
            data.push((255.99 * (pixel.y()).sqrt().clamp(0.0, 0.999)) as u8);
            data.push((255.99 * (pixel.z()).sqrt().clamp(0.0, 0.999)) as u8);
        }
    }
    bar.finish();

    writer.write_image_data(&data).unwrap();
}

fn ray_color(scene: &Scene, r: &Ray, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let hit_record = r.hit(scene);
    match hit_record {
        Some(t) => {
            let scattered = t.material.scatter(r, &t);
            match scattered {
                Ok(scattered_ray) => {
                    return scattered_ray.0 * ray_color(scene, &scattered_ray.1, depth - 1)
                }
                Err(_) => return Vec3::new(0.0, 0.0, 0.0),
            }
        }
        _ => {
            let unit_direction: Vec3 = unit_vector(r.direction);
            let t = 0.5 * (unit_direction.y() + 1.0);
            return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}
