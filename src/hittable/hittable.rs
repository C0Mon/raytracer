use crate::render::ray::Ray;
use crate::{Vec3, Point3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vec3, t: f64, front_face: bool) -> Self {
        Self { point, normal, t, front_face }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
            return;
        }
        self.normal = -outward_normal;
    }

    pub fn update(&mut self, rec: HitRecord) {
        self.point = rec.point;
        self.normal = rec.normal;
        self.t = rec.t;
        self.front_face = rec.front_face;
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord::new(
            Point3::default(),
            Vec3::default(),
            0.0,
            true
        )
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.update(temp_rec);
            }
        }

        hit_anything
    }
}

impl Default for HittableList {
    fn default() -> Self {
        HittableList { objects: Vec::new() }
    }
}
