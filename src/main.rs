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
mod util;
mod vec3;

const MAX_DEPTH: u32 = 50;
const SAMPLES_PER_PIXEL: u32 = 100;

fn main() {
    let path = Path::new("image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let aspect_ratio = 3.0 / 2.0;

    let image_width: u32 = 1200;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    let mut encoder = png::Encoder::new(w, image_width, image_height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::Fast);

    let mut writer = encoder.write_header().unwrap();

    let mut data: Vec<u8> = vec![];

    let bar = ProgressBar::new(image_height as u64);

    let scene = Scene::random_scene();

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

                        let r: Ray = camera.get_ray(u, v);

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
