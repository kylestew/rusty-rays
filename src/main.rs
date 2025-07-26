use minifb::{Key, Window, WindowOptions};

mod core;
mod shapes;

use core::camera::Camera;
use core::hittable_list::HittableList;
use core::Point3;
use shapes::sphere::Sphere;

fn main() -> Result<(), std::io::Error> {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::default(800, 16.0 / 9.0);

    // Allocate a BGRA-encoded frame buffer
    let mut buffer: Vec<u32> = vec![0; camera.image_width * camera.image_height];

    camera.render(&mut buffer, &world);

    let mut window = Window::new(
        "Raytracer ‑ ESC to exit",
        camera.image_width,
        camera.image_height,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, camera.image_width, camera.image_height)
            .unwrap();
    }

    Ok(())
}
