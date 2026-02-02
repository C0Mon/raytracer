
pub mod math;
pub mod hittable;
pub mod render;

use std::sync::Arc;

pub use math::vector::{Vec3, Point3};
pub use hittable::{Hittable, Sphere};
pub use render::colour::Colour;

use crate::{hittable::{hittable::HittableList, material::{Lambertian, Metal}}, render::camera::Camera};

pub fn run () -> std::io::Result<()> {

    // World
    let mut world = HittableList::default();

    // Materials
    let material_ground = Arc::new(Lambertian::new(&Colour::new(0.8, 0.8, 0.0)));
    let material_centre = Arc::new(Lambertian::new(&Colour::new(0.5, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(&Colour::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(&Colour::new(0.8, 0.6, 0.2), 1.0));
    
    // Objects
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
        
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2), 0.5, material_centre)));
        
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
        
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));
        

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let mut cam = Camera::new(aspect_ratio, image_width, 100, 10);

    cam.render(&world)?;
    Ok(())
}

