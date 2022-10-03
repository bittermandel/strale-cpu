use glam::Vec3;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}

pub struct SolidColor {
    pub color_value: Vec3,
}

impl SolidColor {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self {
            color_value: Vec3::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        self.color_value
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture + Send + Sync>,
    even: Box<dyn Texture + Send + Sync>,
}

impl CheckerTexture {
    pub fn new_from_colors(c1: Vec3, c2: Vec3) -> Self {
        Self {
            even: Box::new(SolidColor::new(c1.x, c1.y, c1.z)),
            odd: Box::new(SolidColor::new(c2.x, c2.y, c2.z)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct TightCheckerTexture {
    odd: Box<dyn Texture + Send + Sync>,
    even: Box<dyn Texture + Send + Sync>,
}

impl TightCheckerTexture {
    #[allow(dead_code)]
    pub fn new_from_colors(c1: Vec3, c2: Vec3) -> Self {
        Self {
            even: Box::new(SolidColor::new(c1.x, c1.y, c1.z)),
            odd: Box::new(SolidColor::new(c2.x, c2.y, c2.z)),
        }
    }
}

impl Texture for TightCheckerTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (100.0 * p.x).sin() * (100.0 * p.y).sin() * (100.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
