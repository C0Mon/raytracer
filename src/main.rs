mod render;
mod math;

use std::{fs::File};
use render::ppm::{Ppm};
use render::ray::{Ray};
use math::vector::{Point3, Vec3};

fn ray_colour(r: &Ray) -> Colour{
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {

    let aspect_ratio = 16.0 / 9.0;
    let image_width: isize = 400;

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

    let file = File::create("raycast.ppm")?;
    let mut image = Ppm::new("P3", 256, 256);

    for x in 0..image.height {
        for y in 0..image.width {
            let pixel_centre = pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
            let ray_direction = pixel_centre - camera_centre;
            let r = Ray::new(&camera_centre, &ray_direction);
        
            image.set_pixel(x, y, ray_colour(&r));
        }
    }


    // write_gradient(&mut image);
    image.write_image(file)?;
    
    Ok(())
}


use render::colour::{Colour};
fn write_gradient(image: &mut Ppm) -> &Ppm{
    for x in 0..image.height {
        for y in 0..image.width {
            image.set_pixel(x, y, Colour::new(x as f64, y as f64, 0.0 ));
        }
    }
    image
}
