use super::{Color, HitRecord, Ray};
use crate::materials::lambertian::Lambertian;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MaterialDef {
    Lambertian { albedo: Color },
    // Metal { albedo: Color, fuxx: f64 }
    // ....
}

impl MaterialDef {
    pub fn into_material(self) -> Arc<dyn Material> {
        match self {
            MaterialDef::Lambertian { albedo } => Arc::new(Lambertian { albedo }),
        }
    }
}

pub struct MatBounce {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MatBounce>;
}
