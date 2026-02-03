use std::fmt::Debug;
use std::sync::{Arc, LazyLock};

use crate::hittable::Material;
use crate::hittable::material::Lambertian;
use crate::math::interval::Interval;
use crate::render::ray::Ray;
use crate::{Colour, Point3, Vec3};

static DEFAULT_MATERIAL: LazyLock<Arc<dyn Material + Send + Sync>> =
    LazyLock::new(|| Arc::new(Lambertian::new(&Colour::new(0.0, 0.0, 0.0))));

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material + Send + Sync>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: Point3,
        normal: Vec3,
        t: f64,
        front_face: bool,
        mat: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        Self {
            point,
            normal,
            mat,
            t,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
            return;
        }
        self.normal = -outward_normal;
    }

    pub fn update(&mut self, rec: &HitRecord) {
        self.point = rec.point;
        self.normal = rec.normal;
        self.t = rec.t;
        self.front_face = rec.front_face;
        self.mat = rec.mat.clone();
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord::new(
            Point3::default(),
            Vec3::default(),
            0.0,
            true,
            DEFAULT_MATERIAL.clone(),
        )
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.update(&temp_rec);
            }
        }

        hit_anything
    }
}
