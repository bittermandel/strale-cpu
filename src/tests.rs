#![feature(test)]






/// Creates `n` deterministic random cubes. Returns the `Vec` of surface `Triangle`s.

#[cfg(feature = "bench")]
pub fn intersect_list(objects: Vec<Box<(dyn Hittable)>>, b: &mut ::test::Bencher) {
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

    b.iter(|| {
        // Iterate over the list of triangles.
        for object in &objects {
            object.hit(&ray, 0.001, f32::MAX);
        }
    });
}

#[cfg(feature = "bench")]
#[bench]
/// Benchmark intersecting a random scene directly.
fn bench_random_scene(b: &mut ::test::Bencher) {
    let mut scene = Scene::new();
    scene.randomize();
    intersect_list(scene.objects, b);
}

#[cfg(feature = "bench")]
#[bench]
/// Benchmark intersecting a random scene with a bvh.
fn bench_random_scene_with_bvh(b: &mut ::test::Bencher) {
    use crate::bvh::Bvh;

    let mut scene = Scene::new();
    scene.randomize();
    scene.objects = vec![Box::new(Bvh::new(scene.objects, 0.0, f32::MAX))];
    intersect_list(scene.objects, b);
}

#[cfg(feature = "bench")]
#[bench]
/// Benchmark creating a random scene and BVH.
fn create_scene_and_bvh(b: &mut ::test::Bencher) {
    use crate::bvh::Bvh;

    b.iter(|| {
        let mut scene = Scene::new();
        scene.randomize();
        Bvh::new(scene.objects, 0.0, f32::MAX);
    });
}
