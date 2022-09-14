use crate::geometry::Geometry;

pub struct Scene {
    pub objects: Vec<Geometry>,
}

impl Scene {
    pub fn new(objects: Vec<Geometry>) -> Scene {
        Scene { objects }
    }

    pub fn add(&mut self, object: Geometry) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn objects(&self) -> &Vec<Geometry> {
        return &self.objects;
    }
}
