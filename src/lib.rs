// Declare top-level modules
pub mod math;
// pub mod ray;
pub mod hittable;
pub mod render;

use std::fs::File;

// Re-export commonly used types
// pub use ray::Ray;
pub use math::vector::{Vec3, Point3};
pub use hittable::{Hittable, Sphere};
pub use render::colour::Colour;

use crate::render::{ppm::Ppm, ray::Ray};

fn hit_sphere(centre: Point3, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = centre - r.origin();
    let a = r.direction().dot(r.direction());
    let h = r.direction().dot(oc);
    let c = oc.dot(oc) - (radius * radius);

    let discriminant =  h * h - a * c;
    
    if discriminant < 0.0 {
        return -1.0;
    }
    (h - discriminant.sqrt()) / a
}

fn ray_colour(r: &Ray) -> Colour{
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let N: Vec3 = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Colour::new(N.x + 1.0, N.y + 1.0, N.z + 1.0);
    }
    
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
}

pub fn run () -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;

    // Calculate image height, and ensure that it's at least 1
    let mut image_height:isize = (image_width as f64 / aspect_ratio) as isize;
    if image_height < 1  {
        image_height = 1;
    }

    // camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64/image_height as f64);
    let camera_centre = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left = camera_centre - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    println!("{}", image_height);
    println!("{}", image_width);
    let file = File::create("raycast.ppm")?;
    let mut image = Ppm::new("P3", image_width, image_height as usize);

    for row in 0..image.height {
        for col in 0..image.width {
            let pixel_centre = pixel00_loc + (col as f64 * pixel_delta_u) + (row as f64 * pixel_delta_v);
            let ray_direction = pixel_centre - camera_centre;
            let r = Ray::new(&camera_centre, &ray_direction);
        
            image.set_pixel(row, col, ray_colour(&r));
        }
    }


    // write_gradient(&mut image);
    image.write_image(file)?;
    
    Ok(())
}

