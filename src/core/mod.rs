pub mod hittable;
pub mod ray;
pub mod vec3;

// Re-export commonly used types for convenience
pub use hittable::HitRecord;
pub use ray::Ray;
pub use vec3::{Point3, Vec3};
