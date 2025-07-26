use super::hittable::{HitRecord, Hittable};
use super::interval::Interval;
use super::{Color, Point3, Ray, Vec3};

pub struct Camera {
    // pub aspect_ratio: f64,
    pub image_width: usize,
    pub image_height: usize,

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
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, buf: &mut Vec<u32>, world: &dyn Hittable) {
        for y in 0..self.image_height {
            // print!("\rScanlines remaining: {} ", image_height - j);
            for x in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (x as f64 * self.pixel_delta_u)
                    + (y as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);
                let pixel_color = self.ray_color(&r, world);

                // write color
                let ir = (255.999 * pixel_color.x) as u32;
                let ig = (255.999 * pixel_color.y) as u32;
                let ib = (255.999 * pixel_color.z) as u32;
                // writeln!(writer, "{} {} {}", ir, ig, ib)?;
                buf[y * self.image_width + x] = (255 << 24) | (ir << 16) | (ig << 8) | ib;
            }
        }
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
