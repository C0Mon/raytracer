mod render;
mod math;

use std::{fs::File};

fn main() -> std::io::Result<()> {
    let file = File::create("image.ppm")?;
    let mut image = Ppm::new("P3", 256, 256);
    write_gradient(&mut image);
    image.write_image(file)?;
    
    Ok(())
}

use render::ppm::{Ppm};
use render::colour::{Colour};
fn write_gradient(image: &mut Ppm) -> &Ppm{
    for x in 0..image.height {
        for y in 0..image.width {
            image.set_pixel(x, y, Colour::new(x as f64, y as f64, 0.0 ));
        }
    }
    image
}
