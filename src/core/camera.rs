use super::hittable::{HitRecord, Hittable};
use super::interval::Interval;
use super::{Color, Point3, Ray, Vec3};
use rand::Rng;

pub struct Camera {
    pub image_width: usize,
    pub image_height: usize,
    pub samples_per_pixel: usize,

    center: Point3, // camera center

    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn default(image_width: usize, aspect_ratio: f64) -> Self {
        // Image
        let height = ((image_width as f64) / aspect_ratio) as usize;
        let image_height: usize = if height > 1 { height } else { 1 };

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Point3::new(0.0, 0.0, 0.0);

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

        let viewport_upper_left = center
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            }
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            // aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel: 10,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render_pixel(&self, world: &dyn Hittable, x: usize, y: usize) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..self.samples_per_pixel {
            let r = self.get_ray(x, y);
            color += self.ray_color(&r, world);
        }
        color / self.samples_per_pixel as f64
    }

    fn get_ray(&self, x: usize, y: usize) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((x as f64 + offset.x) * self.pixel_delta_u)
            + ((y as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.0)
    }

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(ray, Interval::new(0.0, f64::INFINITY), &mut rec) {
            // simple normal-mapped shading
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_dir = Vec3::unit_vector(ray.direction);
        let a = 0.5 * (unit_dir.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
