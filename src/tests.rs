#![feature(test)]

use glam::Vec3A;
use rand::Rng;

use crate::{camera::Camera, hittable::Hittable, ray::Ray, scene::Scene};

/// Creates `n` deterministic random cubes. Returns the `Vec` of surface `Triangle`s.

#[cfg(feature = "bench")]
pub fn traverse(bvh: BVH, shapes: &[Box<dyn Hittable>], b: &mut ::test::Bencher) {
    let aspect_ratio = 3.0 / 2.0;

    let image_width: f32 = 1080 as f32;
    let image_height: f32 = image_width as f32 / aspect_ratio;

    let lookfrom = Vec3A::new(13.0, 2.0, 3.0);
    let lookat = Vec3A::new(0.0, 0.0, 0.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3A::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    let mut rng = rand::thread_rng();

    let u: f32 = rng.gen_range(0.0..image_width);
    let v: f32 = rng.gen_range(0.0..image_height);

    let ray: Ray = camera.get_ray(u, v);

    bvh.traverse(&ray, shapes);
}

#[cfg(feature = "bench")]
#[bench]
/// Benchmark creating a random scene and BVH.
fn create_scene_and_bvh(b: &mut ::test::Bencher) {
    use crate::bvh::Bvh;
    let mut scene = Scene::new();
    scene.randomize();
    let bvh = BVH::build(&scene.objects);

    b.iter(|| {
        traverse(bvh, &scene.objects, b);
    });
}
