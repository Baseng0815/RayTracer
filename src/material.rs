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
    pub albedo: Vec3,
    pub fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, intersect: &Intersect) -> (Ray, Vec3) {
        let reflected = ray_in.direction.reflected(&intersect.normal);
        let fuzzed_direction = (reflected + Vec3::random_on_unit_sphere() * self.fuzz).normalized();

        (Ray {
            origin: intersect.point,
            direction: fuzzed_direction
        }, self.albedo)
    }
}

pub struct Dielectric {
    pub eta: f64
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, intersect: &Intersect) -> (Ray, Vec3) {
        let is_front_face = -intersect.normal.dot(&ray_in.direction) >= 0.0;
        let normal = if is_front_face { intersect.normal } else { -intersect.normal };
        let frac_eta = if is_front_face { 1.0 / self.eta } else { self.eta / 1.0 };
        let refracted = ray_in.direction.refracted(&normal, frac_eta).normalized();

        (Ray {
            origin: intersect.point,
            direction: refracted
        }, Vec3::UNIT)
    }
}
