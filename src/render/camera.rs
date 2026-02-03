use std::{fs::File};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use crate::{Colour, Point3, Vec3, hittable::hittable::{HitRecord, HittableList}, math::{interval::Interval, util::{degrees_to_radians, random_normalised}}, render::{ppm::Ppm, ray::Ray}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: usize,     // Rendered image width in pixel count
    pub samples_per_pixel: u32, // Count of random samples for each pixel
    pub max_depth: u32,         // Maximum number of ray bounces into scene


    pub vfov: f64,          // Vertical view angle (field of view)
    pub lookfrom: Point3,   // Point camera is looking from
    pub lookat: Point3,     // Point camera is looking at
    pub vup: Vec3,          // Camera-relative "up" direction

    pub defocus_angle: f64, // Variation angle of rays through each pixel
    pub focus_dist: f64,    // Distance from camera lookfrom point to plane of perfect focus

    image_height: isize,        // Rendered image height
    pixel_sample_scale: f64,    // Color scale factor for a sum of pixel samples
    centre: Point3,             // Camera center
    pixel00_loc: Point3,        // Location of pixel 0, 0
    pixel_delta_u: Vec3,        // Offset to pixel to the right
    pixel_delta_v: Vec3,        // Offset to pixel below

    // Camera frame basis vectors
    u: Vec3,                    
    v: Vec3,
    w: Vec3,

    defocus_disk_u: Vec3,   // Defocus disk horizontal radius
    defocus_disk_v: Vec3,   // Defocus disk vertical radius
}

impl Camera {
    pub fn new() -> Self {
        let mut cam = Self { 
            aspect_ratio: 1.0, 
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,

            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),

            defocus_angle: 0.0,
            focus_dist: 10.0,

            image_height: 0,
            pixel_sample_scale: 0.0,
            centre: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),

            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),

            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
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
        let defocus_angle = self.defocus_angle;
        let defocus_disk_u = self.defocus_disk_u;
        let defocus_disk_v = self.defocus_disk_v;

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
                        // Set origin
                        let ray_origin: Vec3;
                        if defocus_angle <= 0.0 {
                            ray_origin = centre;
                        }
                        else {
                            let p = Vec3::random_in_unit_disk();
                            ray_origin = centre + (p.x * defocus_disk_u) + (p.y * defocus_disk_v);
                        }
                        // Sample 
                        let offset = Camera::sample_square();
                        let pixel_sample = pixel00
                            + ((col as f64 + offset.x) * delta_u)
                            + ((row as f64 + offset.y) * delta_v);
                        // Set colour
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
        self.image_height = (self.image_width as f64 / self.aspect_ratio).round() as isize;
        if self.image_height < 1  {
            self.image_height = 1;
        }
        
        // Camera
        self.pixel_sample_scale = 1.0 / (self.samples_per_pixel as f64);
        self.centre = self.lookfrom;

        // Determine viewport dimensions
        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta/2.0);

        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64/self.image_height as f64);

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame.
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(self.w).unit_vector();
        self.v = self.w.cross(self.u);

        // Calculate the vectors across the horizontal and down the vertical edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;  // Vector across viewport horizontal edge
        self.pixel_delta_v = viewport_v / self.image_height as f64; // Vector down viewport vertical edge

        // Calculate the location of the upper left pixel
        let viewport_upper_left = self.centre - (self.focus_dist * self.w) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * f64::tan(degrees_to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_colour(r: &Ray, world: &HittableList, depth: u32) -> Colour {
        // If ray has exceeded bounce limit, no more light is gathered

        if depth <= 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        // Object
        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Colour::default();
        
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Self::ray_colour(&scattered, world, depth-1);
            }
            return Colour::new(0.0, 0.0, 0.0);
            
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