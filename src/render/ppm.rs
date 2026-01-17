use std::{fs::File, io::{self, BufWriter, Write}};
use super::colour::{Colour};


pub struct Ppm {
    pub image_type: String,
    pub width: usize,
    pub height: usize,
    pub max_val: usize,
    pub image: Vec<Vec<Colour>>,
}

impl Ppm {
    pub fn new(image_type: &str, width: usize, height: usize) -> Self {
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
    
    pub fn set_pixel(&mut self, row: usize, col: usize, pixel: Colour) {
        self.image[row][col] = pixel;
    }

    pub fn get_headers(&self) -> String {
        format!("{0}\n{1} {2}\n{3}\n", self.image_type, self.width, self.height, self.max_val)
    }

    pub fn write_image(&self, file: File) -> io::Result<()> {
        let mut writer = BufWriter::new(file);
        writer.write_all(&(self.get_headers()).into_bytes())?;
        for x in 0..self.height {
            eprint!("\rScanlines remaining: {} ", self.height - x);
            io::stderr().flush().unwrap();
            for y in 0..self.width {
                let pixel: Colour = self.image[x][y];
                pixel.write_colour(&mut writer)?;
            }
        }
        writer.flush()?;
        eprint!("\r\x1b[2KDone\n");
        Ok(())
    }
}
