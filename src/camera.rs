use crate::{vec::Vec3, ray::Ray};

pub struct Camera {
    pub height: u32,
    pub aspect_ratio: f64,
    pub width: u32,
    focal_length: f64,
    vp_width: f64,
    vp_height: f64,
    pub samples_per_pixel: u32,
    origin: Vec3,
    pixel_delta_x: Vec3,
    pixel_delta_y: Vec3,
    vp_pixel00: Vec3
}

impl Camera {
    pub fn new(height: u32, aspect_ratio: f64, focal_length: f64, vp_height: f64,
               samples_per_pixel: u32, origin: Vec3) -> Camera {
        let width = ((height as f64) * aspect_ratio) as u32;
        let vp_width = vp_height * aspect_ratio;
        let vp_horizontal = Vec3::RIGHT * vp_width;
        let vp_vertical = -Vec3::UP * vp_height;
        let lower_left_corner = origin - vp_horizontal * 0.5 - vp_vertical * 0.5 + Vec3::FORWARD * focal_length;
        let pixel_delta_x = vp_horizontal * (1.0 / width as f64);
        let pixel_delta_y = vp_vertical * (1.0 / height as f64);
        let vp_pixel00 = lower_left_corner + (pixel_delta_x + pixel_delta_y) * 0.5;

        Camera {
            height,
            aspect_ratio,
            width,
            focal_length,
            vp_width,
            vp_height,
            samples_per_pixel,
            origin,
            pixel_delta_x,
            pixel_delta_y,
            vp_pixel00
        }
    }

    pub fn generate_ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        let pixel_center = self.vp_pixel00 + (self.pixel_delta_x * x as f64 + self.pixel_delta_y * y as f64);
        let sample_off_x = rand::random::<f64>() - 0.5;
        let sample_off_y = rand::random::<f64>() - 0.5;
        let pixel_sample = pixel_center + (self.pixel_delta_x * sample_off_x +
                                           self.pixel_delta_y * sample_off_y);

        Ray {
            origin: self.origin,
            direction: (pixel_sample - self.origin).normalized()
        }
    }
}
