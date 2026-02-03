
pub mod math;
pub mod hittable;
pub mod render;

use std::sync::Arc;

pub use math::vector::{Vec3, Point3};
pub use hittable::{Hittable, Sphere};
pub use render::colour::Colour;

use crate::{hittable::{Material, hittable::HittableList, material::{Dielectric, Lambertian, Metal}}, math::util::{random_float, random_normalised}, render::camera::Camera};

pub fn run () -> std::io::Result<()> {

    // World
    let mut world = HittableList::default();

    // Materials
    let material_ground = Arc::new(Lambertian::new(&Colour::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0), 1000.0, material_ground)));


    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_normalised();
            let centre = Point3::new(a as f64 + 0.9 * random_normalised(), 0.2, b as f64 + 0.9 * random_normalised());

            if (centre - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Colour::random() * Colour::random();
                    sphere_material = Arc::new(Lambertian::new(&albedo));
                    world.add(Box::new(Sphere::new(centre, 0.2, sphere_material)));
                }
                else if choose_mat < 0.95 {
                    let albedo = Colour::random() * Colour::random();
                    let fuzz = random_float(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(&albedo, fuzz));
                    world.add(Box::new(Sphere::new(centre, 0.2, sphere_material)));
                }
                else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(centre, 0.2, sphere_material)));
                }
            }
        }
    }


    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(&Colour::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(&Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0), 1.0, material3)));


    // Camera
    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Point3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    cam.render(&world)?;
    Ok(())
}

