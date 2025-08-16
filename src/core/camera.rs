use crate::core::Point3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Camera {
    pub position: Point3, // look from
    pub target: Point3,   // look at

    pub fov: f64,

    pub samples_per_pixel: usize,
    pub max_depth: usize,

    pub image_width: usize,
    pub aspect_ratio: f64,
}
