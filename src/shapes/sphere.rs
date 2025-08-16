use crate::core::hittable::{HitRecord, Hittable};
use crate::core::interval::Interval;
use crate::core::material::Material;
use crate::core::{Point3, Ray, Vec3};
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = Vec3::dot(r.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let mut root = (h - discriminant.sqrt()) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let hit_p = r.at(root);
        let hit_t = root;
        let ray_d = r.direction;
        let outward_normal = (hit_p - self.center) / self.radius;
        let mat = Arc::clone(&self.mat);

        Some(HitRecord::new(hit_p, hit_t, ray_d, outward_normal, mat))
    }
}
