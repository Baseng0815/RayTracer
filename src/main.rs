use std::rc::Rc;

use image::Rgb;

mod vec;
mod ray;
mod geometry;
mod camera;
mod scene;
mod raytracer;
mod material;

use vec::*;
use geometry::*;
use camera::*;
use scene::*;
use raytracer::*;
use material::*;

fn main() {
    let material_ground: Rc<dyn Material>   = Rc::new(Lambertian { albedo: Vec3(0.8, 0.8, 0.0) });
    let material_center: Rc<dyn Material>   = Rc::new(Lambertian { albedo: Vec3(0.7, 0.3, 0.3) });
    let material_left: Rc<dyn Material>     = Rc::new(Metal      { albedo: Vec3(0.8, 0.8, 0.8) });
    let material_right: Rc<dyn Material>    = Rc::new(Metal      { albedo: Vec3(0.8, 0.6, 0.2) });

    let mut scene = Scene::new();
    scene.geometry.push(Box::new(Sphere { origin: Vec3(0.0, -100.5, -1.0), radius: 100.0, material: Rc::clone(&material_ground) }));
    scene.geometry.push(Box::new(Sphere { origin: Vec3(0.0,    0.0, -1.0), radius: 0.5,   material: Rc::clone(&material_center) }));
    scene.geometry.push(Box::new(Sphere { origin: Vec3(-1.0,   0.0, -1.0), radius: 0.5,   material: Rc::clone(&material_left) }));
    scene.geometry.push(Box::new(Sphere { origin: Vec3(1.0,    0.0, -1.0), radius: 0.5,   material: Rc::clone(&material_right) }));

    let camera = Camera::new(480, 16.0 / 9.0, 1.0, 2.0, 100, Vec3::ZERO);
    let raytracer = Raytracer::new(Vec3(0.5, 0.7, 1.0), 50);

    let mut buf: image::ImageBuffer<Rgb<u8>, _> = image::ImageBuffer::new(camera.width, camera.height);

    for y in 0..camera.height {
        println!("Scanlines remaining: {}/{}", y, camera.height);
        for x in 0..camera.width {
            let mut color_sum = Vec3::ZERO;

            for _ in 0..camera.samples_per_pixel {
                let ray = camera.generate_ray_for_pixel(x, y);
                color_sum += raytracer.ray_color(&ray, &scene);
            }

            color_sum *= 1.0 / camera.samples_per_pixel as f64;
            // gamma correction
            color_sum = Vec3(color_sum.0.sqrt(), color_sum.1.sqrt(), color_sum.2.sqrt());

            buf.put_pixel(x, y, color_sum.into());
        }
    }

    if let Err(e) = buf.save("out.jpg") {
        println!("{}", e.to_string());
    }
}
