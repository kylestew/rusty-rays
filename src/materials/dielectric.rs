use crate::core::material::{MatBounce, Material};
use crate::core::{Color, HitRecord, Ray, Vec3};
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MatBounce> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::unit_vector(r_in.direction);
        let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let scattered;
        if cannot_refract || Dielectric::reflectance(cos_theta, ri) > rand::rng().random() {
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
