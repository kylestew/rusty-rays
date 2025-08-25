use crate::core::hittable::HitRecord;
use crate::core::material::{MatBounce, Material};
use crate::core::math::random_unit_vector;
use crate::core::ray::Ray;
use glam::Vec3;
use rand::RngCore;

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        let fuzz = fuzz.clamp(0.0, 1.0);
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut dyn RngCore) -> Option<MatBounce> {
        let mut reflected = Vec3::reflect(r_in.direction, rec.normal);
        reflected = reflected.normalize() + (self.fuzz * random_unit_vector(rng));

        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;

        if Vec3::dot(scattered.direction, rec.normal) > 0.0 {
            Some(MatBounce {
                attenuation,
                scattered,
            })
        } else {
            None
        }
    }
}
