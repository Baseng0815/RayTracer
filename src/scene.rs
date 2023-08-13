use crate::geometry::*;
use crate::ray::*;

pub struct Scene {
    pub geometry: Vec<Box<dyn Geometry>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            geometry: Vec::new()
        }
    }

    pub fn cast_ray(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersect> {
        let mut closest = None;

        for geom in &self.geometry {
            if let Some(intersect) = geom.intersect_with(ray, t_min, t_max) {
                if closest == None {
                    closest = Some(intersect)
                } else if intersect.t < closest.unwrap().t {
                    closest = Some(intersect);
                }
            }
        }

        closest
    }
}
