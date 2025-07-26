use minifb::{Key, Window, WindowOptions};

mod core;
mod shapes;

use core::hittable::{HitRecord, Hittable};
use core::hittable_list::HittableList;
use core::interval::Interval;
use core::{Point3, Ray, Vec3};
use shapes::sphere::Sphere;
use std::f64;

type Color = Vec3;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(ray, Interval::new(0.0, f64::INFINITY), &mut rec) {
        // simple normal-mapped shading
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_dir = Vec3::unit_vector(ray.direction);
    let a = 0.5 * (unit_dir.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), std::io::Error> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 800;
    let height = ((image_width as f64) / aspect_ratio) as usize;
    let image_height: usize = if height > 1 { height } else { 1 };

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let viewport_v = Vec3 {
        x: 0.0,
        y: -viewport_height,
        z: 0.0,
    };

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        }
        - viewport_u / 2.0
        - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Allocate a BGRA-encoded frame buffer
    let mut buffer: Vec<u32> = vec![0; image_width * image_height];

    // --- render loop ---
    for y in 0..image_height {
        // print!("\rScanlines remaining: {} ", image_height - j);
        for x in 0..image_width {
            let pixel_center =
                pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r, &world);

            // write color
            let ir = (255.999 * pixel_color.x) as u32;
            let ig = (255.999 * pixel_color.y) as u32;
            let ib = (255.999 * pixel_color.z) as u32;
            // writeln!(writer, "{} {} {}", ir, ig, ib)?;
            buffer[y * image_width + x] = (255 << 24) | (ir << 16) | (ig << 8) | ib;
        }
    }

    let mut window = Window::new(
        "Raytracer ‑ ESC to exit",
        image_width,
        image_height,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, image_width, image_height)
            .unwrap();
    }

    Ok(())
}
