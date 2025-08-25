#![cfg(not(target_arch = "wasm32"))]

mod core;
mod materials;
mod primitives;
mod scenes;

use crate::core::math::vec_to_rgb_u32;
use crate::core::renderer::Renderer;
// use crate::core::scene::SceneDef;

use minifb::{Key, Window, WindowOptions};
// rand::RngCore trait is pulled in via type of &mut rng at callsite

fn main() -> Result<(), std::io::Error> {
    // // build the scene
    // let file = File::open("scene.json")?;
    // let def: SceneDef = serde_json::from_reader(file)?;
    // let scene = def.build();
    // build the scene (hand-built random spheres)
    let scene = scenes::random_world();

    // create a renderer
    let renderer = Renderer::from_camera(&scene.camera);

    // Display Window
    let mut window = Window::new(
        "Raytracer ‑ ESC to exit",
        renderer.image_width,
        renderer.image_height,
        WindowOptions::default(),
    )
    .unwrap();

    // Allocate a BGRA-encoded frame buffer
    let mut buffer = vec![0u32; renderer.image_width * renderer.image_height];

    // // === PARALLEL RENDER =====================================================
    // // Share immutable world across threads
    // let world = Arc::new(scene.world);
    //
    // buffer
    //     .par_chunks_mut(renderer.image_width) // each chunk = one scanline
    //     .enumerate()
    //     .for_each(|(y, row)| {
    //         for x in 0..renderer.image_width {
    //             let c = renderer.render_pixel(world.as_ref(), x, y);
    //             row[x] = c.to_rgb_u32();
    //         }
    //         println!("{}", y);
    //     });
    // // =========================================================================

    let mut rng = rand::rng();

    // render pixel by pixel, flushing buffer to display as we go
    for y in 0..renderer.image_height {
        for x in 0..renderer.image_width {
            let c = renderer.render_pixel(&scene.world, x, y, &mut rng);
            buffer[y * renderer.image_width + x] = vec_to_rgb_u32(c);
        }

        window
            .update_with_buffer(&buffer, renderer.image_width, renderer.image_height)
            .unwrap();

        // give the window a chance to process events each scan‑line
        if !window.is_open() || window.is_key_down(Key::Escape) {
            return Ok(());
        }
    }

    // RENDERING COMPLETE!
    // keep window open and present the final frame each tick until user quits
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, renderer.image_width, renderer.image_height)
            .unwrap();
    }

    Ok(())
}
