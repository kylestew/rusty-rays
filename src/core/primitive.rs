use crate::core::hittable::Hittable;
use crate::core::material::MaterialDef;
use crate::primitives::sphere::Sphere;
use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub enum PrimitiveDef {
    Sphere {
        center: Vec3,
        radius: f32,
        material: MaterialDef,
    },
    // Plane { ... }
    // QUAD
    // MESH
}

impl PrimitiveDef {
    pub fn into_hittable(self) -> Box<dyn Hittable> {
        match self {
            PrimitiveDef::Sphere {
                center,
                radius,
                material,
            } => {
                let mat = material.into_material();
                Box::new(Sphere::new(center, radius, mat))
            }
        }
    }
}
