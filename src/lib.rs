use wasm_bindgen::prelude::*;

mod core;
mod shapes;

use core::camera::Camera;
use core::hittable_list::HittableList;
use core::Point3;
use shapes::sphere::Sphere;

#[wasm_bindgen]
pub struct RustyRays {
    camera: Camera,
    world: HittableList,
}

#[wasm_bindgen]
impl RustyRays {
    pub fn new(width: usize, aspect: f64, samples: usize) -> RustyRays {
        let mut world = HittableList::new();
        world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

        let mut camera = Camera::default(width, aspect);
        camera.samples_per_pixel = samples;

        RustyRays { camera, world }
    }

    /// Trace one pixel and return packed BGRA.
    #[wasm_bindgen]
    pub fn trace_pixel(&self, x: usize, y: usize) -> u32 {
        let c = self.camera.render_pixel(&self.world, x, y);
        c.to_rgb_u32()
    }

    #[wasm_bindgen(getter)]
    pub fn image_width(&self) -> usize {
        self.camera.image_width
    }

    #[wasm_bindgen(getter)]
    pub fn image_height(&self) -> usize {
        self.camera.image_height
    }
}
