use crate::core::camera::Camera;
use crate::core::hittable::Hittable;
use crate::core::interval::Interval;
use crate::core::{Color, Ray, Vec3};
use rand::Rng;

#[derive(Debug)]
pub struct Renderer {
    pub image_width: usize,
    pub image_height: usize,

    samples_per_pixel: usize,

    max_depth: usize,

    camera: Camera,
}

impl Renderer {
    pub fn from_camera(camera: &Camera) -> Self {
        Self {
            image_width: camera.image_width,
            image_height: camera.image_height,

            samples_per_pixel: camera.samples_per_pixel,
            max_depth: camera.max_depth,

            camera: *camera,
        }
    }

    pub fn render_pixel(&self, world: &dyn Hittable, x: usize, y: usize) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..self.samples_per_pixel {
            let r = self.get_ray(x, y);
            color += self.ray_color(&r, self.max_depth, world);
        }
        color / self.samples_per_pixel as f64
    }

    fn get_ray(&self, x: usize, y: usize) -> Ray {
        // Construct a camera ray directed at a randomly sampled point around the pixel location.
        let offset = self.sample_square();
        self.camera.ray_through_pixel_with_offset(x, y, offset)
    }

    fn sample_square(&self) -> Vec3 {
        let mut rng = rand::rng();
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(rng.random::<f64>() - 0.5, rng.random::<f64>() - 0.5, 0.0)
    }

    fn ray_color(&self, ray: &Ray, depth: usize, world: &dyn Hittable) -> Color {
        // we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        // shoot a ray into the world, see if we hit anything
        if let Some(hit_rec) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            // hit an object in the world
            // use it's material to determine ray bounce behavior
            if let Some(bounce) = hit_rec.mat.scatter(ray, &hit_rec) {
                return bounce.attenuation * self.ray_color(&bounce.scattered, depth - 1, world);
            }

            return Color::default();
        }

        let unit_dir = Vec3::unit_vector(ray.direction);
        let a = 0.5 * (unit_dir.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
