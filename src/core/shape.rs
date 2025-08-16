use crate::core::hittable::Hittable;
use crate::core::material::MaterialDef;
use crate::core::Point3;
use crate::shapes::sphere::Sphere;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ShapeDef {
    Sphere {
        center: Point3,
        radius: f64,
        material: MaterialDef,
    }, // Plane { ... }
       // Triangle { ... }
}

impl ShapeDef {
    pub fn into_hittable(self) -> Box<dyn Hittable> {
        match self {
            ShapeDef::Sphere {
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
