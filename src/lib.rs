
pub mod math;
pub mod hittable;
pub mod render;


pub use math::vector::{Vec3, Point3};
pub use hittable::{Hittable, Sphere};
pub use render::colour::Colour;

use crate::{hittable::hittable::HittableList, render::camera::Camera};

pub fn run () -> std::io::Result<()> {

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5
    ))
    );
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0
    ))
    );

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let mut cam = Camera::new(aspect_ratio, image_width);

    cam.render(&world)?;
    Ok(())
}

