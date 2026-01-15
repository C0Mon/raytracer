use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("image.ppm")?;
    let image = write_gradient();
    let out = image.format();
    file.write_all(&out.into_bytes())?;
    Ok(())
}

use ppm::{Ppm, Pixel};
fn write_gradient() -> Ppm{
    let mut image = Ppm::new("P3", 256, 256);
    for x in 0..image.height {
        for y in 0..image.width {
            image.set_pixel(x, y, Pixel { r: x as u8, g: y as u8, b: 0 });
        }
    }
    image
}

mod ppm {
    pub struct Ppm {
        pub image_type: String,
        pub height: usize,
        pub width: usize,
        pub max_val: usize,
        pub image: Vec<Vec<Pixel>>,
    }

    impl Ppm {
        pub fn new(image_type: &str, height: usize, width: usize) -> Self {
            let image = vec![
                vec![Pixel { r: 0, g: 0, b: 0 }; width];
                height
            ];
            Self {
                image_type: image_type.to_string(),
                width,
                height,
                max_val: 255,
                image,
            }
        }

        pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
            self.image[y][x] = pixel;
        }
        
        pub fn format(&self) -> String {
            let mut format_img = format!("{0}\n{1} {2}\n{3}\n", self.image_type, self.height, self.width, self.max_val);
            for y in 0..self.height {
                for x in 0..self.width {
                    let pixel_str = self.image[y][x].format();
                    format_img.push_str(&pixel_str);
                }
            }
            return format_img
        }
    }

    #[derive(Clone)]
    pub struct Pixel {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl Pixel {
        pub fn format(&self) -> String{
            format!("{0} {1} {2}\n", self.r, self.g, self.b)
        }
    }
}