use super::hittable::HitRecord;
use super::ray::Ray;
use crate::materials::*;
use glam::Vec3;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub enum MaterialDef {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { ir: f32 },
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
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut dyn RngCore) -> Option<MatBounce>;
}
