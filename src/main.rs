use std::sync::{Arc, RwLock, Mutex};
use std::thread;

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
    let mat_ground:      Arc<dyn Material + Send + Sync>   = Arc::new(Lambertian { albedo: Vec3(0.8, 0.8, 0.0) });
    let mat_center:      Arc<dyn Material + Send + Sync>   = Arc::new(Lambertian { albedo: Vec3(0.7, 0.3, 0.3) });
    let mat_left:        Arc<dyn Material + Send + Sync>   = Arc::new(Metal      { albedo: Vec3(0.8, 0.8, 0.8), fuzz: 0.3 });
    let mat_right:       Arc<dyn Material + Send + Sync>   = Arc::new(Metal      { albedo: Vec3(0.8, 0.6, 0.2), fuzz: 0.1 });
    let mat_dielectric:  Arc<dyn Material + Send + Sync>   = Arc::new(Dielectric { eta: 1.5 });

    let mut scene = Scene::new();
    scene.geometry.push(Box::new(Sphere { origin: Vec3( 0.0, -100.5, -1.0), radius: 100.0, material: Arc::clone(&mat_ground) }));
    scene.geometry.push(Box::new(Sphere { origin: Vec3(-0.5,    0.2, -1.5), radius:   0.5, material: Arc::clone(&mat_center) }));
    scene.geometry.push(Box::new(Sphere { origin: Vec3(-1.0,    0.0, -1.0), radius:   0.5, material: Arc::clone(&mat_dielectric) }));
    scene.geometry.push(Box::new(Sphere { origin: Vec3( 1.0,    0.0, -1.0), radius:   0.5, material: Arc::clone(&mat_right) }));

    let camera = Camera::new(480, 16.0 / 9.0, 1.0, 2.0, 50, Vec3::ZERO);
    let raytracer = Raytracer::new(Vec3(0.5, 0.7, 1.0), 10);

    let thread_count = 24;
    let thread_scanlines = camera.height / thread_count;

    let scene_arc = Arc::new(scene);
    let camera_arc = Arc::new(camera);
    let raytracer_arc = Arc::new(raytracer);
    let buf_arc: Arc<Mutex<image::ImageBuffer<Rgb<u8>, _>>> = Arc::new(Mutex::new(image::ImageBuffer::new(camera_arc.width, camera_arc.height)));

    let mut join_handles = Vec::new();
    for ty in (0..camera_arc.height).step_by(thread_scanlines as usize) {
        println!("Launching thread #{} processing scanlines {}-{}...", ty / thread_scanlines, ty, ty + thread_scanlines);
        let scene_inner = Arc::clone(&scene_arc);
        let camera_inner = Arc::clone(&camera_arc);
        let raytracer_inner = Arc::clone(&raytracer_arc);
        let buf_inner = Arc::clone(&buf_arc);

        join_handles.push(thread::spawn(move || {
            for y in ty..(ty + thread_scanlines) {
                for x in 0..camera_inner.width {
                    let mut color_sum = Vec3::ZERO;

                    for _ in 0..camera_inner.samples_per_pixel {
                        let ray = camera_inner.generate_ray_for_pixel(x, y);
                        color_sum += raytracer_inner.ray_color(&ray, &scene_inner);
                    }

                    color_sum *= 1.0 / camera_inner.samples_per_pixel as f64;
                    // gamma correction
                    color_sum = Vec3(color_sum.0.sqrt(), color_sum.1.sqrt(), color_sum.2.sqrt());

                    let mut buf_lock = buf_inner.lock().unwrap();
                    buf_lock.put_pixel(x, y, color_sum.into());
                }

            }
        }));
    }

    for join_handle in join_handles {
        join_handle.join().expect("couldn't join thread");
    }

    let buf_lock = buf_arc.lock().unwrap();
    if let Err(e) = buf_lock.save("out.jpg") {
        println!("{}", e.to_string());
    }
}
