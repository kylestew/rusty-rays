use crate::core::material::{MatBounce, Material};
use crate::core::{Color, HitRecord, Ray, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<MatBounce> {
        // Cosine‑weighted random bounce
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

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
