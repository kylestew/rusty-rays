use crate::core::ray::Ray;
use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CameraDef {
    pub position: Vec3, // look from
    pub target: Vec3,   // look at

    pub fov: f32,

    pub samples_per_pixel: usize,
    pub max_depth: usize,

    pub image_width: usize,
    pub aspect_ratio: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub image_width: usize,
    pub image_height: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,

    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl CameraDef {
    pub fn to_camera(&self) -> Camera {
        // Image
        let image_width = self.image_width;
        let image_height = ((image_width as f32) / self.aspect_ratio).round().max(1.0) as usize;

        // Determine viewport dimensions
        let focal_length = (self.position - self.target).length();
        let theta = self.fov.to_radians();
        let half_height_tan = (theta / 2.0).tan();
        let viewport_height = 2.0 * half_height_tan * focal_length;
        let viewport_width = viewport_height * self.aspect_ratio;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame
        let vup = Vec3::new(0., 1., 0.);
        let w = (self.position - self.target).normalize();
        let u = Vec3::cross(vup, w).normalize();
        let v = Vec3::cross(w, u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let center = self.position;
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = center - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_width,
            image_height,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}

impl Camera {
    pub fn ray_through_pixel_with_offset(&self, x: usize, y: usize, offset: Vec3) -> Ray {
        let pixel_sample = self.pixel00_loc
            + ((x as f32 + offset.x) * self.pixel_delta_u)
            + ((y as f32 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    pub fn ray_through_pixel(&self, x: usize, y: usize) -> Ray {
        self.ray_through_pixel_with_offset(x, y, Vec3::new(0.0, 0.0, 0.0))
    }
}
