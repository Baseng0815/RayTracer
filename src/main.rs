use image::{GenericImage, Rgb};

mod vec;
mod ray;
mod geometry;
mod camera;

use vec::*;
use ray::*;
use geometry::*;
use camera::*;

type Scene = Vec<Box<dyn Geometry>>;

// map ray to background color
fn ray_color(scene: &Scene, ray: &Ray) -> Vec3 {
    for geometry in scene {
        if let Some(intersect) = geometry.intersect_with(&ray, 0.0, f64::MAX) {
            let normal = &intersect.normal;
            return Vec3(normal.0 + 1.0, normal.1 + 1.0, normal.2 + 1.0) * 0.5;
        }
    }

    let a = 0.5 * (ray.direction.1 + 1.0);
    Vec3::UNIT * (1.0 - a) + Vec3(0.5, 0.7, 1.0) * a
}

fn main() {
    // scene
    let scene: Vec<Box<dyn Geometry>> = vec![
        Box::new(Sphere { origin: Vec3(0.0, 0.0, -1.0), radius: 0.5 }),
        Box::new(Sphere { origin: Vec3(0.0, -100.5, -1.0), radius: 100.0 })
    ];

    let camera = Camera::new(1080, 16.0 / 9.0, 1.0, 2.0, Vec3::ZERO);

    let mut buf: image::ImageBuffer<Rgb<u8>, _> = image::ImageBuffer::new(camera.width, camera.height);

    for y in 0..camera.height {
        // println!("Scanlines remaining: {}/{}", y, img_height);
        for x in 0..camera.width {
            let ray = camera.generate_ray_for_pixel(x, y);

            let pix = ray_color(&scene, &ray).into();
            buf.put_pixel(x, y, pix);
        }
    }

    if let Err(e) = buf.save("out.jpg") {
        println!("{}", e.to_string());
    }
}
