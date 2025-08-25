mod core;
mod materials;
mod primitives;

#[cfg(target_arch = "wasm32")]
use crate::core::renderer::Renderer;
#[cfg(target_arch = "wasm32")]
use crate::core::scene::{Scene, SceneDef};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct RustyRays {
    renderer: Renderer,
    scene: Scene,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl RustyRays {
    pub fn new(from: &str) -> Result<RustyRays, JsValue> {
        let def: SceneDef = serde_json::from_str(from)
            .map_err(|e| JsValue::from_str(&format!("Invalid scene JSON: {}", e)))?;
        let scene = def.build();

        // create a renderer
        let renderer = Renderer::from_camera(&scene.camera);

        Ok(RustyRays { renderer, scene })
    }

    /// Trace one pixel and return packed BGRA.
    #[wasm_bindgen]
    pub fn render_pixel(&self, x: usize, y: usize) -> u32 {
        let c = self.renderer.render_pixel(&self.scene.world, x, y);
        // Desktop uses 0x00RRGGBB for minifb. For wasm, return 0xFFBBGGRR so JS can
        // write directly into a Uint32Array view of ImageData with no per-channel unpack.
        let rgb = c.to_rgb_u32(); // 0x00RRGGBB
        let r = (rgb >> 16) & 0xff;
        let g = (rgb >> 8) & 0xff;
        let b = rgb & 0xff;
        (0xff << 24) | (b << 16) | (g << 8) | r
    }

    #[wasm_bindgen(getter)]
    pub fn image_width(&self) -> usize {
        self.renderer.image_width
    }

    #[wasm_bindgen(getter)]
    pub fn image_height(&self) -> usize {
        self.renderer.image_height
    }
}
