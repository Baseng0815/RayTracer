use image::{GenericImage, Rgb};

mod vec;
mod ray;

use vec::*;
use ray::*;

// map ray to background color
fn ray_color(ray: &Ray) -> Vec3 {
    let t = 0.5 * (ray.1.1 + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f64 / aspect_ratio) as u32;

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - Vec3(0.0, 0.0, focal_length);

    let mut buf: image::ImageBuffer<Rgb<u8>, _> = image::ImageBuffer::new(img_width, img_height);

    for y in 0..img_height {
        println!("Scanlines remaining: {}/{}", y, img_height);
        for x in 0..img_width {
            let u = y as f64 / (img_height - 1) as f64;
            let v = x as f64 / (img_width - 1) as f64;
            let ray = Ray::new(&origin, &(lower_left_corner + u * vertical + v * horizontal));
            let pix = ray_color(&ray).into();
            buf.put_pixel(x, img_height - y - 1, pix);
        }
    }

    buf.save("out.jpg");
}
