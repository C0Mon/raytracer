
pub mod math;
pub mod hittable;
pub mod render;

use std::sync::Arc;

pub use math::vector::{Vec3, Point3};
pub use hittable::{Hittable, Sphere};
pub use render::colour::Colour;

use crate::{hittable::{hittable::HittableList, material::{Dielectric, Lambertian, Metal}}, render::camera::Camera};

pub fn run () -> std::io::Result<()> {

    // World
    let mut world = HittableList::default();

    // Materials
    let material_ground = Arc::new(Lambertian::new(&Colour::new(0.8, 0.8, 0.0)));
    let material_centre = Arc::new(Lambertian::new(&Colour::new(0.5, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right = Arc::new(Metal::new(&Colour::new(0.8, 0.6, 0.2), 1.0));
    
    // Objects
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
        
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2), 0.5, material_centre)));
        
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
        
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
        
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));
        

    // Camera
    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 10;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Point3::new(0.0, 1.0, 0.0);

    cam.render(&world)?;
    Ok(())
}

