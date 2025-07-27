#![cfg(not(target_arch = "wasm32"))]

use minifb::{Key, Window, WindowOptions};

mod core;
mod shapes;

use core::camera::Camera;
use core::hittable_list::HittableList;
use core::vec3::to_bgra_u32;
use core::Point3;
use shapes::sphere::Sphere;

fn main() -> Result<(), std::io::Error> {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut camera = Camera::default(800, 16.0 / 9.0);
    camera.samples_per_pixel = 10;

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
            buffer[y * camera.image_width + x] = to_bgra_u32(c);
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
