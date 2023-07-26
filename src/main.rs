use image::{GenericImage, Rgb};

mod vec;
mod ray;
mod geometry;

use vec::*;
use ray::*;
use geometry::*;

// map ray to background color
fn ray_color(ray: &Ray) -> Vec3 {
    let t = 0.5 * (ray.direction.y + 1.0);
    Vec3::UNIT * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // scene
    let sphere = Sphere {
        origin: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5
    };

    // image
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f64 / aspect_ratio) as u32;
    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::ZERO;
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal * 0.5 + vertical * 0.5 + Vec3::FORWARD * focal_length);

    let mut buf: image::ImageBuffer<Rgb<u8>, _> = image::ImageBuffer::new(img_width, img_height);

    for y in 0..img_height {
        println!("Scanlines remaining: {}/{}", y, img_height);
        for x in 0..img_width {
            let u = y as f64 / (img_height - 1) as f64;
            let v = x as f64 / (img_width - 1) as f64;
            let ray = Ray::new(&origin, &(lower_left_corner + vertical * u + horizontal * v));
            let pix = ray_color(&ray).into();
            buf.put_pixel(x, img_height - y - 1, pix);
        }
    }

    if let Err(e) = buf.save("out.jpg") {
        println!("{}", e.to_string());
    }
}
