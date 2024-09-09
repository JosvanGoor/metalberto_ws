use super::Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    pub origin:    Vec3,
    pub direction: Vec3,
}

impl Ray {

    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, distance: f64) -> Vec3 {
        self.origin + (self.direction * distance)
    }

}