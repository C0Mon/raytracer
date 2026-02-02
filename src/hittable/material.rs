use std::fmt::Debug;
use crate::{Colour, Vec3, hittable::hittable::HitRecord, math::interval::Interval, render::ray::Ray};


pub trait Material: Debug + Send + Sync {
    fn scatter (&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool;
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Colour
} 

impl Lambertian {
    pub fn new(albedo: &Colour) -> Self {
        Self {albedo: *albedo}
    }
}

impl Material for Lambertian {
    fn scatter (&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(&rec.point, &scatter_direction);
        *attenuation = self.albedo;
        return true

    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    fuzz: f64,
    albedo: Colour
} 

impl Material for Metal {
    fn scatter (&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool {
        let mut reflected = r_in.direction().unit_vector().reflect(&rec.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());
        *scattered = Ray::new(&rec.point, &reflected);
        *attenuation = self.albedo;
        return scattered.direction().dot(rec.normal) > 0.0;
    }
}

impl Metal {
    pub fn new(albedo: &Colour, fuzz: f64) -> Self {
        let fuzz_range = Interval::new(0.0, 1.0);
        Self {albedo: *albedo, fuzz: fuzz_range.clamp(fuzz)}
    }
}
