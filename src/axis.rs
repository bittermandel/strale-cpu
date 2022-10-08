use std::ops::Index;

use glam::Vec3A;

#[derive(Copy, Clone, Debug)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

/// Make slices indexable by `Axis`.
impl Index<Axis> for Vec3A {
    type Output = f32;

    fn index(&self, axis: Axis) -> &f32 {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}
