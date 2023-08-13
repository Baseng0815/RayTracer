use image::{GenericImage, Rgb};

mod vec;
mod ray;
mod geometry;
mod camera;
mod scene;

use vec::*;
use geometry::*;
use camera::*;
use scene::*;

fn main() {
    // scene
    let mut scene = Scene::new();
    scene.geometry.push(Box::new(Sphere { origin: Vec3(0.0, 0.0, -1.0), radius: 0.5 }));
    scene.geometry.push(Box::new(Sphere { origin: Vec3(0.0, -100.5, -1.0), radius: 100.0 }));

    let camera = Camera::new(480, 16.0 / 9.0, 1.0, 2.0, 10, Vec3::ZERO);

    let mut buf: image::ImageBuffer<Rgb<u8>, _> = image::ImageBuffer::new(camera.width, camera.height);

    for y in 0..camera.height {
        // println!("Scanlines remaining: {}/{}", y, img_height);
        for x in 0..camera.width {
            let mut color_sum = Vec3::ZERO;

            for _ in 0..camera.samples_per_pixel {
                let ray = camera.generate_ray_for_pixel(x, y);
                let hit = scene.cast_ray(&ray, 0.0, f64::MAX);
                let color = match hit {
                    Some(intersect) => (intersect.normal + Vec3::UNIT) * 0.5,
                    None => Camera::background_color(&ray)
                };

                color_sum += color;
            }

            color_sum *= 1.0 / camera.samples_per_pixel as f64;

            buf.put_pixel(x, y, color_sum.into());
        }
    }

    if let Err(e) = buf.save("out.jpg") {
        println!("{}", e.to_string());
    }
}
