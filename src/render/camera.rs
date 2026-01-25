use std::{fs::File};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use crate::{Colour, Point3, Vec3, hittable::hittable::{HitRecord, HittableList}, math::{interval::Interval, util::random_normalised}, render::{ppm::Ppm, ray::Ray}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    image_height: isize,
    pixel_sample_scale: f64, // Color scale factor for a sum of pixel samples
    centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize, samples_per_pixel: u32, max_depth: u32) -> Self {
        let mut cam = Self { 
            aspect_ratio, 
            image_width,
            samples_per_pixel,
            max_depth,
            image_height: 0,
            pixel_sample_scale: 0.0,
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

        // Copy data to avoid using self in pariter
        let pixel00 = self.pixel00_loc;
        let delta_u = self.pixel_delta_u;
        let delta_v = self.pixel_delta_v;
        let samples = self.samples_per_pixel;
        let scale = self.pixel_sample_scale;
        let centre = self.centre;
        let max_depth = self.max_depth;

        let file = File::create("raycast.ppm")?;
        let mut image = Ppm::new("P3", self.image_width, self.image_height as usize);

        // Create a progress bar
        let pb = ProgressBar::new((self.image_height * self.image_width as isize) as u64, );
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"));

        let pixels: Vec<(usize, usize, Colour)> = (0..self.image_height as usize)
            .into_par_iter()                     // convert to parallel iterator
            .flat_map(|row| {
                (0..self.image_width).into_par_iter().progress_with(pb.clone()).map(move |col| {
                    let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);

                    for _ in 0..samples {
                        let ray_origin = centre;
                        let offset = Camera::sample_square();
                        let pixel_sample = pixel00
                            + ((col as f64 + offset.x) * delta_u)
                            + ((row as f64 + offset.y) * delta_v);

                        let r = Ray::new(&ray_origin, &(pixel_sample - ray_origin));
                        pixel_colour += Camera::ray_colour(&r, world, max_depth);
                    }

                    let final_colour = scale * pixel_colour;
                    (row, col, final_colour)
                })
            })
            .collect(); // collect all pixel results

        for (row, col, colour) in pixels {
            image.set_pixel(row, col, colour);
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
        self.pixel_sample_scale = 1.0 / (self.samples_per_pixel as f64);
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

    fn ray_colour(r: &Ray, world: &HittableList, mut depth: u32) -> Colour {
        // If ray has exceeded bounce limit, no more light is gathered

        if depth <= 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        // Object
        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let direction = rec.normal + Vec3::random_unit_vector();
            depth -= 1;
            return 0.5 * Self::ray_colour(&Ray::new(&rec.point, &direction), world, depth);
        }
        
        // Background
        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
    }

    fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square
        Vec3::new(random_normalised() - 0.5, random_normalised() - 0.5, 0.0)
    }
}