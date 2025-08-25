use super::hittable::{HitRecord, Hittable};
use super::interval::Interval;
use super::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn with_object(obj: Box<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(obj);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if let Some(hit_rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit_rec.t;
                temp_rec = Some(hit_rec);
            }
        }

        temp_rec
    }
}
