mod render;
mod math;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("image.ppm")?;
    let image = write_gradient();
    let out = image.format();
    file.write_all(&out.into_bytes())?;
    Ok(())
}

use render::ppm::{Ppm, Pixel};
fn write_gradient() -> Ppm{
    let mut image = Ppm::new("P3", 256, 256);
    for x in 0..image.height {
        for y in 0..image.width {
            image.set_pixel(x, y, Pixel { r: x as u8, g: y as u8, b: 0 });
        }
    }
    image
}
