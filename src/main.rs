#![cfg_attr(feature = "bench", feature(test))]

use bvh2::BVH;
use glam::Vec3A;
use rayon::prelude::*;
use std::{fs::File, io::BufWriter, path::Path, time::Instant, vec};

use camera::Camera;
use indicatif::ProgressBar;
use rand::Rng;
use ray::Ray;
use scene::Scene;
use vec3::unit_vector;

mod aabb;
mod axis;
mod bvh;
mod bvh2;
mod camera;
mod geometry;
mod hittable;
mod material;
mod perlin;
mod ray;
mod scene;
mod texture;
mod util;
mod vec3;

#[cfg(all(feature = "bench", test))]
extern crate test;

#[cfg(test)]
mod tests;

const MAX_DEPTH: u32 = 16;
const SAMPLES_PER_PIXEL: u32 = 100;

fn main() {
    let path = Path::new("image.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let aspect_ratio = 3.0 / 2.0;

    let image_width: u32 = 1080;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;

    let lookfrom = Vec3A::new(13.0, 2.0, 3.0);
    let lookat = Vec3A::new(0.0, 0.0, 0.0);

    println!(
        "Configuration:\ndepth: {}, samples: {}, image_size:{}x{}",
        MAX_DEPTH, SAMPLES_PER_PIXEL, image_width, image_height
    );

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3A::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.01,
        10.0,
    );

    let mut encoder = png::Encoder::new(w, image_width, image_height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::Fast);

    let mut writer = encoder.write_header().unwrap();

    let mut data: Vec<u8> = vec![];

    let bar = ProgressBar::new((image_height * image_width).into());

    let mut scene = Scene::new();
    scene.randomize();

    let bvh = bvh2::BVH::build(&mut scene.objects);

    let mut pixelvecs: Vec<Vec<Vec3A>> = vec![];

    let now = Instant::now();

    (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            let pixels: Vec<Vec3A> = (0..image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Vec3A::new(0.0, 0.0, 0.0);

                    let mut rng = rand::thread_rng();

                    for _ in 0..SAMPLES_PER_PIXEL {
                        let u = (i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
                        let v = (j as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;

                        let r: Ray = camera.get_ray(u, v);

                        let color = ray_color(&scene, &r, &bvh, MAX_DEPTH);
                        pixel_color += color;
                    }
                    bar.inc(1);
                    let scale = 1.0 / SAMPLES_PER_PIXEL as f32;

                    pixel_color * scale
                })
                .collect();

            pixels
        })
        .collect_into_vec(&mut pixelvecs);

    println!("{} in {:?} seconds", bar.per_sec(), now.elapsed());

    for pixelvec in pixelvecs {
        for pixel in pixelvec {
            data.push((255.99 * (pixel.x).sqrt().clamp(0.0, 0.999)) as u8);
            data.push((255.99 * (pixel.y).sqrt().clamp(0.0, 0.999)) as u8);
            data.push((255.99 * (pixel.z).sqrt().clamp(0.0, 0.999)) as u8);
        }
    }
    bar.finish();

    writer.write_image_data(&data).unwrap();
}

fn ray_color(scene: &Scene, r: &Ray, bvh: &BVH, depth: u32) -> Vec3A {
    if depth == 0 {
        return Vec3A::new(0.0, 0.0, 0.0);
    }

    let shapes = bvh.traverse(r, &scene.objects);

    let hit_record = r.hit(shapes);
    match hit_record {
        Some(t) => {
            let scattered = t.material.scatter(r, &t);
            match scattered {
                Ok(scattered_ray) => {
                    scattered_ray.0 * ray_color(scene, &scattered_ray.1, bvh, depth - 1)
                }
                Err(_) => Vec3A::new(0.0, 0.0, 0.0),
            }
        }
        _ => {
            let unit_direction = unit_vector(r.direction);
            let t = 0.5 * (unit_direction.y + 1.0);

            Vec3A::new(1.0, 1.0, 1.0) * (1.0 - t) + t * Vec3A::new(0.5, 0.7, 1.0)

            /*let tex = TightCheckerTexture::new_from_colors(
                Vec3A::new(0.2, 0.3, 0.1),
                Vec3A::new(0.9, 0.9, 0.9),
            );

            let uv = Sphere::get_sphere_uv(unit_vector(r.direction));

            tex.value(uv.0, uv.1, unit_vector(r.direction))*/
        }
    }
}
