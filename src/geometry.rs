use crate::ray::*;
use crate::vec::*;

pub struct Intersect {

}

pub trait Geometry {
    fn intersect_with(&self, ray: &Ray) -> Option<Intersect>;
}

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64
}

impl Geometry for Sphere {
    fn intersect_with(&self, ray: &Ray) -> Option<Intersect> {
        let ray_to_origin = ray.origin - self.origin;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray_to_origin.dot(&ray.direction);
        let c = ray_to_origin.dot(&ray_to_origin) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a  * c;
        return if discriminant >= 0.0 {
            Some(Intersect {  })
        } else {
            None
        }
    }
}

impl Sphere {

}
