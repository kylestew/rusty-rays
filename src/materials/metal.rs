use crate::core::material::{MatBounce, Material};
use crate::core::{Color, HitRecord, Ray, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = fuzz.clamp(0.0, 1.0);
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MatBounce> {
        let mut reflected = Vec3::reflect(r_in.direction, rec.normal);
        reflected = Vec3::unit_vector(reflected) + (self.fuzz * Vec3::random_unit_vector());

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
