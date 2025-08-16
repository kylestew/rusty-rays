pub mod camera;
pub mod hittable;
// pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod renderer;
pub mod scene;
pub mod shape;
pub mod vec3;

// Re-export commonly used types for convenience
pub use hittable::HitRecord;
pub use ray::Ray;
pub use vec3::{Color, Point3, Vec3};
