use crate::core::camera::Camera;
use crate::core::hittable::Hittable;
use crate::core::interval::Interval;
use crate::core::math::f32_01;
use crate::core::ray::Ray;
use glam::Vec3;
use rand::{rng, RngCore};

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

    pub fn render_pixel(
        &self,
        world: &dyn Hittable,
        x: usize,
        y: usize,
        rng: &mut dyn RngCore,
    ) -> Vec3 {
        let mut color = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..self.samples_per_pixel {
            let r = self.get_ray(x, y, rng);
            color += self.ray_color(&r, self.max_depth, world, rng);
        }
        color / self.samples_per_pixel as f32
    }

    fn get_ray(&self, x: usize, y: usize, rng: &mut dyn RngCore) -> Ray {
        // Construct a camera ray directed at a randomly sampled point around the pixel location.
        let offset = self.sample_square(rng);
        self.camera.ray_through_pixel_with_offset(x, y, offset)
    }

    fn sample_square(&self, rng: &mut dyn RngCore) -> Vec3 {
        // uniform random in [-0.5, 0.5]
        let x = f32_01(rng) - 0.5;
        let y = f32_01(rng) - 0.5;
        Vec3::new(x, y, 0.0)

        // let mut rng = rand::rng();
        // // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        // Vec3::new(rng.random::<f32>() - 0.5, rng.random::<f32>() - 0.5, 0.0)
    }

    fn ray_color(
        &self,
        ray: &Ray,
        depth: usize,
        world: &dyn Hittable,
        rng: &mut dyn RngCore,
    ) -> Vec3 {
        // we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        // shoot a ray into the world, see if we hit anything
        if let Some(hit_rec) = world.hit(ray, Interval::new(0.001, f32::INFINITY)) {
            // hit an object in the world
            // use it's material to determine ray bounce behavior
            if let Some(bounce) = hit_rec.mat.scatter(ray, &hit_rec, rng) {
                return bounce.attenuation
                    * self.ray_color(&bounce.scattered, depth - 1, world, rng);
            }

            return Vec3::default();
        }

        let unit_dir = ray.direction.normalize();
        let a = 0.5 * (unit_dir.y + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }
}
