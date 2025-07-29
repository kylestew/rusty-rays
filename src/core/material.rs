use super::{Color, HitRecord, Ray, Vec3};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        // Cosine‑weighted random bounce
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        // Build the new ray
        *scattered = Ray::new(rec.p, scatter_direction);

        // Set the attenuation colour
        *attenuation = self.albedo; // copies

        true
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(r_in.direction, rec.normal);

        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        true
    }
}
