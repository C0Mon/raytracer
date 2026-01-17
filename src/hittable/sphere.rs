use crate::{Hittable, Point3, Vec3, hittable::hittable::HitRecord, math::interval::Interval, render::ray::Ray};

pub struct Sphere {
    pub centre: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64) -> Self {
        Self { centre, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = self.centre - r.origin();
        let a = r.direction().dot(r.direction());
        let h = r.direction().dot(oc);
        let c = oc.dot(oc) - (self.radius * self.radius);

        let discriminant =  h * h - a * c;
        
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find nearest root that lies in the accepted range
        let mut root = (h - sqrtd) / a;
        if !ray_t.surround(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surround(root) {
                return false
            }
        }
        rec.t = root;
        rec.point = r.at(rec.t);
        let outward_normal = (rec.point - self.centre) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true
    }
}