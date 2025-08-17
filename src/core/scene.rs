use crate::core::camera::{Camera, CameraDef};
use crate::core::hittable::Hittable;
use crate::core::hittable_list::HittableList;
use crate::core::shape::ShapeDef;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneDef {
    camera: CameraDef,
    objects: Vec<ShapeDef>,
}

pub struct Scene {
    pub camera: Camera,
    pub world: HittableList,
}

impl SceneDef {
    pub fn build(self) -> Scene {
        let objects: Vec<Box<dyn Hittable>> = self
            .objects
            .into_iter()
            .map(ShapeDef::into_hittable)
            .collect();

        let world = HittableList { objects };

        Scene {
            camera: self.camera.to_camera(),
            world,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::material::MaterialDef;
    use crate::core::Vec3;
    use serde_json;

    fn eps(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn assert_vec3(v: &Vec3, x: f64, y: f64, z: f64) {
        assert!(eps(v.x, x) && eps(v.y, y) && eps(v.z, z));
    }

    #[test]
    fn scene_deserialization_and_builds() {
        let json = r#"
{
    "camera": {
        "position": [0, 1, 2],
        "target": [0, 0, -1],
        "fov": 45,
        "samples_per_pixel": 10,
        "max_depth": 10,
        "image_width": 800,
        "aspect_ratio": 1.6
    },
    "objects": [
        {
            "type": "sphere",
            "center": [0, -100.5, -1],
            "radius": 100,
            "material": {
                "type": "lambertian",
                "albedo": [0.8, 0.8, 0.0]
            }
        },
        {
            "type": "sphere",
            "center": [0, 0, -1.2],
            "radius": 0.5,
            "material": {
                "type": "lambertian",
                "albedo": [0.1, 0.2, 0.5]
            }
        }
    ]
}
        "#;

        let def: SceneDef = serde_json::from_str(json).expect("Failed to deserialize");

        // Camera
        assert_vec3(&def.camera.position, 0.0, 1.0, 2.0);
        assert_vec3(&def.camera.target, 0.0, 0.0, -1.0);
        assert!(eps(def.camera.fov, 45.0));
        assert_eq!(def.camera.samples_per_pixel, 10);
        assert_eq!(def.camera.max_depth, 10);
        assert_eq!(def.camera.image_width, 800);
        assert!(eps(def.camera.aspect_ratio, 1.6));

        // Objects length
        assert_eq!(def.objects.len(), 2);

        // First sphere
        match &def.objects[0] {
            ShapeDef::Sphere {
                center,
                radius,
                material,
            } => {
                assert_vec3(center, 0.0, -100.5, -1.0);
                assert!(eps(*radius, 100.0));
                match material {
                    MaterialDef::Lambertian { albedo } => {
                        assert_vec3(albedo, 0.8, 0.8, 0.0);
                    }
                    _ => panic!(),
                }
            }
            _ => panic!("expected first object to be a sphere"),
        }

        // Second sphere
        match &def.objects[1] {
            ShapeDef::Sphere {
                center,
                radius,
                material,
            } => {
                assert_vec3(center, 0.0, 0.0, -1.2);
                assert!(eps(*radius, 0.5));
                match material {
                    MaterialDef::Lambertian { albedo } => {
                        assert_vec3(albedo, 0.1, 0.2, 0.5);
                    }
                    _ => panic!(),
                }
            }
            _ => panic!("expected second object to be a sphere"),
        }

        // Build runtime scene (Arc<dyn Hittable> root). Should not panic.
        let built = def.build();
        assert_eq!(built.camera.image_width, 800);
        assert_eq!(built.camera.max_depth, 10);
    }
}
