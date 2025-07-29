#![cfg(not(target_arch = "wasm32"))]

use minifb::{Key, Window, WindowOptions};
use std::rc::Rc;

mod core;
mod shapes;

use core::camera::Camera;
use core::hittable_list::HittableList;
use core::material::{Lambertian, Metal};
use core::{Color, Point3};
use shapes::sphere::Sphere;

fn main() -> Result<(), std::io::Error> {
    // World
    let mat_ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let mat_center = Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    let mat_left = Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
    };
    let mat_right = Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
    };

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(mat_ground),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        Rc::new(mat_center),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(mat_left),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(mat_right),
    )));

    // Camera
    let mut camera = Camera::default(800, 16.0 / 9.0);
    camera.samples_per_pixel = 40;
    camera.max_depth = 10;

    // Display Window
    let mut window = Window::new(
        "Raytracer ‑ ESC to exit",
        camera.image_width,
        camera.image_height,
        WindowOptions::default(),
    )
    .unwrap();

    // Allocate a BGRA-encoded frame buffer
    let mut buffer = vec![0u32; camera.image_width * camera.image_height];

    // render pixel by pixel, flushing buffer to display as we go
    for y in 0..camera.image_height {
        for x in 0..camera.image_width {
            let c = camera.render_pixel(&world, x, y);
            buffer[y * camera.image_width + x] = c.to_rgb_u32();
        }

        window
            .update_with_buffer(&buffer, camera.image_width, camera.image_height)
            .unwrap();

        // give the window a chance to process events each scan‑line
        if !window.is_open() || window.is_key_down(Key::Escape) {
            return Ok(());
        }
    }

    // RENDERING COMPLETE!
    // keep window open until user quits
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
    }

    Ok(())
}
