use std::{fs::File, io::BufWriter, path::Path, vec};

use camera::Camera;
use geometry::Geometry;
use indicatif::ProgressBar;
use ray::Ray;
use scene::Scene;
use vec3::{random_in_unit_sphere, unit_vector, Vec3};

mod camera;
mod geometry;
mod ray;
mod scene;
mod vec3;

const MAX_DEPTH: u32 = 50;
const samples_per_pixel: u32 = 100;

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

    let scene = Scene::new(vec![
        Geometry::Sphere(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Geometry::Sphere(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ]);

    for j in (0..image_height).rev() {
        bar.inc(1);
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

            for a in 0..samples_per_pixel {
                let u = (i as f32 + rand::random::<f32>()) / (image_width - 1) as f32;
                let v = (j as f32 + rand::random::<f32>()) / (image_height - 1) as f32;

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

            let scale = 1.0 / samples_per_pixel as f32;

            data.push((255.99 * (pixel_color.x() * scale).sqrt()) as u8);
            data.push((255.99 * (pixel_color.y() * scale).sqrt()) as u8);
            data.push((255.99 * (pixel_color.z() * scale).sqrt()) as u8);
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
            let target = t.p + t.normal + random_in_unit_sphere();
            return 0.5
                * ray_color(
                    scene,
                    &Ray {
                        origin: t.p,
                        direction: target - t.p,
                    },
                    depth - 1,
                );
        }
        _ => {
            let unit_direction: Vec3 = unit_vector(r.direction);
            let t = 0.5 * (unit_direction.y() + 1.0);
            return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}
