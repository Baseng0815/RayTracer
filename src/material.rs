use crate::vec::Vec3;
use crate::geometry::Intersect;
use crate::ray::Ray;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, intersect: &Intersect) -> (Ray, Vec3);
}

pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, intersect: &Intersect) -> (Ray, Vec3) {
        let mut direction = (intersect.normal + Vec3::random_on_unit_sphere()).normalized();

        if direction.near_zero(1e-8) {
            direction = intersect.normal;
        }

        (Ray {
            origin: intersect.point,
            direction
        }, self.albedo)
    }
}

pub struct Metal {
    pub albedo: Vec3
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, intersect: &Intersect) -> (Ray, Vec3) {
        let mut direction = ray_in.direction.reflected(&intersect.normal);

        (Ray {
            origin: intersect.point,
            direction
        }, self.albedo)
    }
}
