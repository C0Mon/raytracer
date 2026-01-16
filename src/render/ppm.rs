use std::{fs::File, io::{self, BufWriter, Write}};
use super::colour::{Colour};


pub struct Ppm {
    pub image_type: String,
    pub height: usize,
    pub width: usize,
    pub max_val: usize,
    pub image: Vec<Vec<Colour>>,
}

impl Ppm {
    pub fn new(image_type: &str, height: usize, width: usize) -> Self {
        let image = vec![
            vec![Colour::new(0.0, 0.0, 0.0); width];
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
    
    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Colour) {
        self.image[y][x] = pixel;
    }

    pub fn format(&self) -> String {
        use std::io::{self, Write};
        let mut format_img = format!("{0}\n{1} {2}\n{3}\n", self.image_type, self.height, self.width, self.max_val);
        for y in 0..self.height {
            eprint!("\rScanlines remaining: {} ", self.height - y);
            io::stderr().flush().unwrap();
            for x in 0..self.width {
                let pixel_str = self.image[y][x].format();
                format_img.push_str(&pixel_str);
            }
        }
        eprint!("\r\x1b[2KDone\n");
        return format_img
    }

    pub fn get_headers(&self) -> String {
        format!("{0}\n{1} {2}\n{3}\n", self.image_type, self.height, self.width, self.max_val)
    }

    pub fn write_image(&self, file: File) -> io::Result<()> {
        let mut writer = BufWriter::new(file);
        writer.write_all(&(self.get_headers()).into_bytes())?;
        for x in 0..self.height {
            eprint!("\rScanlines remaining: {} ", self.height - x);
            io::stderr().flush().unwrap();
            for y in 0..self.width {
                let mut pixel = self.image[x][y];
                pixel *= 255.99;
                writer.write_all(&(pixel.format()).into_bytes())?;
            }
        }
        writer.flush()?;
        eprint!("\r\x1b[2KDone\n");
        Ok(())
    }
}
