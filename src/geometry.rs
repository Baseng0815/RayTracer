use std::rc::Rc;
use std::sync::Arc;

use crate::ray::*;
use crate::vec::*;
use crate::material::*;

pub struct Intersect {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material + Send + Sync>
}

pub trait Geometry {
    fn intersect_with(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersect>;
}

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material + Send + Sync>
}

impl Geometry for Sphere {
    fn intersect_with(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersect> {
        let oc = ray.origin - self.origin;
        let half_b = oc.dot(&ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - c;

        if discriminant >= 0.0 {
            let mut t = -(half_b + discriminant.sqrt());
            if t < t_min {
                t = -(half_b - discriminant.sqrt());
            }

            if t >= t_min && t <= t_max {
                let point = ray.at(t);
                let normal = (point - self.origin).normalized();
                return Some(Intersect { t, point, normal, material: Arc::clone(&self.material) });
            }
        }

        None
    }
}
