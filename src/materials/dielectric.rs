use crate::core::hittable::HitRecord;
use crate::core::material::{MatBounce, Material};
use crate::core::math::f32_01;
use crate::core::ray::Ray;
use glam::Vec3;
use rand::RngCore;

#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    pub refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut dyn RngCore) -> Option<MatBounce> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction.normalize();
        let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let scattered;
        if cannot_refract || Dielectric::reflectance(cos_theta, ri) > f32_01(rng) {
            scattered = Ray::new(rec.p, Vec3::reflect(unit_direction, rec.normal));
        } else {
            scattered = Ray::new(rec.p, Vec3::refract(unit_direction, rec.normal, ri));
        }

        Some(MatBounce {
            attenuation,
            scattered,
        })
    }
}
