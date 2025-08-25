use super::interval::Interval;
use super::material::Material;
use super::ray::Ray;
use glam::Vec3;
use std::sync::Arc;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        hit_p: Vec3,
        hit_t: f32,
        ray_d: Vec3,
        outward_normal: Vec3,
        mat: Arc<dyn Material>,
    ) -> Self {
        let front_face = Vec3::dot(ray_d, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p: hit_p,
            normal,
            mat,
            t: hit_t,
            front_face,
        }
    }
}
