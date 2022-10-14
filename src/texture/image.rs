use glam::Vec3A;
use image::ImageResult;

use super::Texture;

const BYTES_PER_PIXEL: usize = 3;

#[derive(Debug)]
pub struct ImageTexture {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl ImageTexture {
    pub fn new(path: String) -> Self {
        let data = image::open(path).expect("could not find cubemap");

        Self {
            height: data.height() as usize,
            width: data.width() as usize,
            data: data.as_bytes().to_vec(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, p: glam::Vec3A) -> glam::Vec3A {
        if self.data.is_empty() {
            return Vec3A::new(0.0, 0.0, 0.0);
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let mut i = (u * self.width as f32) as usize;
        let mut j = (v * self.height as f32) as usize;

        if i >= self.width {
            i = self.width - 1;
        }

        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let pixel = BYTES_PER_PIXEL * i + BYTES_PER_PIXEL * self.width * j;

        let r = self.data[pixel] as f32 * color_scale;
        let g = self.data[pixel + 1] as f32 * color_scale;
        let b = self.data[pixel + 2] as f32 * color_scale;

        Vec3A::new(r, g, b)
    }
}
