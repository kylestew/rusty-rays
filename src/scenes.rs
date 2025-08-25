use crate::core::camera::CameraDef;
use crate::core::hittable_list::HittableList;
use crate::core::math::f32_01;
use crate::core::scene::Scene;
use crate::materials::{Dielectric, Lambertian, Metal};
use crate::primitives::sphere::Sphere;
use glam::Vec3;
use rand::rng;
use std::sync::Arc;

pub fn random_world() -> Scene {
    let mut rng = rng();

    let mut world = HittableList::new();

    // Ground
    let ground_material = Arc::new(Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    });
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // Small random spheres grid
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = f32_01(&mut rng);
            let center = Vec3::new(
                a as f32 + 0.9 * f32_01(&mut rng),
                0.2,
                b as f32 + 0.9 * f32_01(&mut rng),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::new(f32_01(&mut rng), f32_01(&mut rng), f32_01(&mut rng))
                        * Vec3::new(f32_01(&mut rng), f32_01(&mut rng), f32_01(&mut rng));
                    let sphere_material = Arc::new(Lambertian { albedo });
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::new(
                        0.5 + 0.5 * f32_01(&mut rng),
                        0.5 + 0.5 * f32_01(&mut rng),
                        0.5 + 0.5 * f32_01(&mut rng),
                    );
                    let fuzz = 0.5 * f32_01(&mut rng);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    // Three large spheres
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    });
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // Camera setup to match the example
    let camera_def = CameraDef {
        position: Vec3::new(13.0, 2.0, 3.0),
        target: Vec3::new(0.0, 0.0, 0.0),
        fov: 20.0,
        samples_per_pixel: 500,
        max_depth: 50,
        image_width: 1200,
        aspect_ratio: 16.0 / 9.0,
        defocus_angle: 0.6,
        focus_dist: 10.0,
    };

    let camera = camera_def.to_camera();

    Scene { camera, world }
}
