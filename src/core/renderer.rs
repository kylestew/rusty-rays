use crate::core::camera::Camera;
use crate::core::hittable::Hittable;
use crate::core::interval::Interval;
use crate::core::{Color, Point3, Ray, Vec3};
use rand::Rng;

#[derive(Debug)]
pub struct Renderer {
    pub image_width: usize,
    pub image_height: usize,

    samples_per_pixel: usize,

    max_depth: usize,

    center: Point3, // camera center
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Renderer {
    pub fn from_camera(camera: &Camera) -> Self {
        // Image
        let image_width = camera.image_width;
        let image_height = ((image_width as f64) / camera.aspect_ratio)
            .round()
            .max(1.0) as usize;

        // Determine viewport dimensions
        let focal_length = (camera.position - camera.target).length();
        let theta = camera.fov.to_radians();
        let half_height_tan = (theta / 2.0).tan();
        let viewport_height = 2.0 * half_height_tan * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame
        let vup = Vec3::new(0., 1., 0.);
        let w = Vec3::unit_vector(camera.position - camera.target);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let center = camera.position;

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = center - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            // aspect_ratio,
            image_width,
            image_height,

            samples_per_pixel: camera.samples_per_pixel,
            max_depth: camera.max_depth,

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
            color += self.ray_color(&r, self.max_depth, world);
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

            Color::default();
        }

        let unit_dir = Vec3::unit_vector(ray.direction);
        let a = 0.5 * (unit_dir.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
