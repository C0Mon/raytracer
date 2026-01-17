use std::fs::File;

use crate::{Colour, Point3, Vec3, hittable::hittable::{HitRecord, HittableList}, math::interval::Interval, render::{ppm::Ppm, ray::Ray}};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    image_height: isize,
    centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
        let mut cam = Self { 
            aspect_ratio, 
            image_width,
            image_height: 0,
            centre: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(), 
        };

        cam.initialise();
        cam
    }

    pub fn render(&mut self, world: &HittableList) -> std::io::Result<()> {
        self.initialise();
        let file = File::create("raycast.ppm")?;
        let mut image = Ppm::new("P3", self.image_width, self.image_height as usize);

        for row in 0..image.height {
            for col in 0..image.width {
                let pixel_centre = self.pixel00_loc + (col as f64 * self.pixel_delta_u) + (row as f64 * self.pixel_delta_v);
                let ray_direction = pixel_centre - self.centre;
                let r = Ray::new(&self.centre, &ray_direction);
            
                image.set_pixel(row, col, Self::ray_colour(&r, &world));
            }
        }

        image.write_image(file)?;
        Ok(())
    }

    fn initialise(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as isize;
        if self.image_height < 1  {
            self.image_height = 1;
        }
        // Camera

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64/self.image_height as f64);
        self.centre = Point3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = self.centre - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_colour(r: &Ray, world: &HittableList) -> Colour{
        let mut rec = HitRecord::default();

        // Object
        if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Colour::new(1.0, 1.0, 1.0));
        }
        
        // Background
        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
    }
}