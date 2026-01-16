fn main() -> std::io::Result<()> {
    raytracer::run()?;
    Ok(())
}

/*
use render::colour::{Colour};
fn write_gradient(image: &mut Ppm) -> &Ppm{
    for x in 0..image.height {
        for y in 0..image.width {
            image.set_pixel(x, y, Colour::new(x as f64, y as f64, 0.0 ));
        }
    }
    image
}

*/
