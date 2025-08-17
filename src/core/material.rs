use super::{Color, HitRecord, Ray};
use crate::materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub enum MaterialDef {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { ir: f64 }, // ....
}

impl MaterialDef {
    pub fn into_material(self) -> Arc<dyn Material> {
        match self {
            MaterialDef::Lambertian { albedo } => Arc::new(Lambertian { albedo }),
            MaterialDef::Metal { albedo, fuzz } => Arc::new(Metal::new(albedo, fuzz)),
            MaterialDef::Dielectric { ir } => Arc::new(Dielectric::new(ir)),
        }
    }
}

pub struct MatBounce {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MatBounce>;
}
