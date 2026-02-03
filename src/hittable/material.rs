use crate::{
    Colour, Vec3,
    hittable::hittable::HitRecord,
    math::{interval::Interval, util::random_normalised},
    render::ray::Ray,
};
use std::fmt::Debug;

pub trait Material: Debug + Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: &Colour) -> Self {
        Self { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(&rec.point, &scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    fuzz: f64,
    albedo: Colour,
}

impl Metal {
    pub fn new(albedo: &Colour, fuzz: f64) -> Self {
        let fuzz_range = Interval::new(0.0, 1.0);
        Self {
            albedo: *albedo,
            fuzz: fuzz_range.clamp(fuzz),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = r_in.direction().unit_vector().reflect(&rec.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());
        *scattered = Ray::new(&rec.point, &reflected);
        *attenuation = self.albedo;
        scattered.direction().dot(rec.normal) > 0.0
    }
}

#[derive(Debug)]
pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Uses Schlick's approximation for reflectance

        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + ((1.0 - r0) * f64::powf(1.0 - cosine, 5.0))
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Colour::new(1.0, 1.0, 1.0);

        let ri: f64 = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = f64::min(-unit_direction.dot(rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - (cos_theta * cos_theta));

        let cannot_refract = ri * sin_theta > 1.0;

        let direction: Vec3 = if cannot_refract || Self::reflectance(cos_theta, ri) > random_normalised() {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(rec.normal, ri)
        };
        *scattered = Ray::new(&rec.point, &direction);
        true
    }
}
