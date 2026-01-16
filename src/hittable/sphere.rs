use crate::{Hittable, Point3, Vec3, hittable::hittable::HitRecord};


pub struct Sphere {
    pub centre: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::render::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
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
        if root <= t_min  || t_max <= root {
            root = (h + sqrtd) / a;
            if (root <= t_min || t_max <= root) {
                return false
            }
        }
        rec.t = root;
        rec.point = r.at(rec.t);
        rec.normal = (rec.point - self.centre) / self.radius;

        true
    }
}