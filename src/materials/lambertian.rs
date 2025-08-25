use crate::core::hittable::HitRecord;
use crate::core::material::{MatBounce, Material};
use crate::core::math::{random_unit_vector, Vec3Ext};
use crate::core::ray::Ray;
use glam::Vec3;
use rand::RngCore;

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, rng: &mut dyn RngCore) -> Option<MatBounce> {
        // Cosine‑weighted random bounce
        let mut scatter_direction = rec.normal + random_unit_vector(rng);

        // catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        // Build the new ray
        let scattered = Ray::new(rec.p, scatter_direction);

        // Set the attenuation colour
        let attenuation = self.albedo; // copies

        Some(MatBounce {
            attenuation,
            scattered,
        })
    }
}
