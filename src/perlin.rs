use glam::{Vec3, Vec3A};
use rand::Rng;
use std::vec;

pub struct Perlin {
    point_count: usize,
    ranfloat: Vec<f32>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let point_count = 256;
        let mut rng = rand::thread_rng();

        fn perlin_generate_perm<R: Rng>(rng: &mut R, point_count: usize) -> Vec<i32> {
            let mut p: Vec<i32> = (0..256).collect();

            for i in (1..point_count).rev() {
                let target = rng.gen_range(0..i);
                p.swap(i, target);
            }

            p
        }

        Self {
            point_count,
            ranfloat: (0..256).map(|_| rng.gen()).collect(),
            perm_x: perlin_generate_perm(&mut rng, point_count),
            perm_y: perlin_generate_perm(&mut rng, point_count),
            perm_z: perlin_generate_perm(&mut rng, point_count),
        }
    }

    pub fn noise(&self, p: Vec3A) -> f32 {
        let i = (4.0 * p.x) as i32 & 255;
        let j = (4.0 * p.y) as i32 & 255;
        let k = (4.0 * p.z) as i32 & 255;

        return self.ranfloat[(self.perm_x[i as usize]
            ^ self.perm_y[j as usize]
            ^ self.perm_z[k as usize]) as usize];
    }
}
