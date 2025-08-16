/*
mod core;
mod shapes;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct RustyRays {
    camera: Camera,
    world: HittableList,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl RustyRays {
    pub fn new(from: JSON) -> RustyRays {
        let mut world = HittableList::new();

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
*/
