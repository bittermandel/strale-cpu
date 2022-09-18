use std::ops::*;

use rand::{thread_rng, Rng};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        return self.e[0];
    }

    pub fn y(&self) -> f32 {
        return self.e[1];
    }

    pub fn z(&self) -> f32 {
        return self.e[2];
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        return self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
    }

    pub fn dot(&self, v: Vec3) -> f32 {
        return self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2];
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return self.e[0].abs() < s && self.e[1].abs() < s && self.e[1].abs() < s;
    }

    pub fn reflect(self, normal: Vec3) -> Vec3 {
        return self - (2.0 * self.dot(normal)) * normal;
    }

    pub fn cross(&self, v: Vec3) -> Vec3 {
        return Vec3::new(
            self.e[1] * v.e[2] - self.e[2] * v.e[1],
            self.e[2] * v.e[0] - self.e[0] * v.e[2],
            self.e[0] * v.e[1] - self.e[1] * v.e[0],
        );
    }

    pub fn random() -> Vec3 {
        let mut rng = thread_rng();

        return Vec3::new(rng.gen(), rng.gen(), rng.gen());
    }

    pub fn random_from(min: f32, max: f32) -> Vec3 {
        let mut rng = thread_rng();

        return Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        );
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        return Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        };
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        return Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        };
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        return Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        };
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        return Vec3 {
            e: [self * v.e[0], self * v.e[1], self * v.e[2]],
        };
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        return Vec3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        };
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        return Vec3 {
            e: [self.e[0] / t, self.e[1] / t, self.e[2] / t],
        };
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        return Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        };
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    return v / v.length();
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    return unit_vector(random_in_unit_sphere());
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = n.dot(-uv).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    return r_out_perp + r_out_parallel;
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = thread_rng();

    loop {
        let p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        };
        return p;
    }
}
