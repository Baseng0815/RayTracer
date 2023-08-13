use crate::ray::*;
use crate::vec::*;

pub struct Camera {
    pub height: u32,
    pub aspect_ratio: f64,
    pub width: u32,
    focal_length: f64,
    vp_width: f64,
    vp_height: f64,
    origin: Vec3,
    vp_horizontal: Vec3,
    vp_vertical: Vec3,
    lower_left_corner: Vec3
}

impl Camera {
    pub fn new(height: u32, aspect_ratio: f64, focal_length: f64, viewport_height: f64, origin: Vec3) -> Camera {
        let width = ((height as f64) * aspect_ratio) as u32;
        let viewport_width = viewport_height * aspect_ratio;
        let viewport_horizontal = Vec3::RIGHT * viewport_width;
        let viewport_vertical = -Vec3::UP * viewport_height;
        let lower_left_corner = origin - viewport_horizontal * 0.5 - viewport_vertical * 0.5 + Vec3::FORWARD * focal_length;

        Camera {
            height,
            aspect_ratio,
            width,
            focal_length,
            vp_width: viewport_width,
            vp_height: viewport_height,
            origin,
            vp_horizontal: viewport_horizontal,
            vp_vertical: viewport_vertical,
            lower_left_corner
        }
    }

    pub fn generate_ray_for_pixel(&self, x: u32, y: u32) -> Ray {
            let u = x as f64 / self.width as f64;
            let v = y as f64 / self.height as f64;

            Ray {
                origin: self.origin,
                direction: (self.lower_left_corner + self.vp_horizontal * u + self.vp_vertical * v).normalized()
            }
    }
}
