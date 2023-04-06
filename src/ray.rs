use crate::vec::*;

pub struct Ray(pub Vec3, pub Vec3);

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray(*origin, direction.normalized())
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.0 + t * self.1
    }
}
