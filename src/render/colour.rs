use std::{ io::{Write, Result}};

use crate::math::{interval::Interval, vector::Vec3};

pub type Colour = Vec3;

impl Colour {
    pub fn write_colour<W: Write>(&self, writer: &mut W) -> Result<()> {
        let intensity = Interval::new(0.0, 0.999);

        let r = (256.0 * intensity.clamp(self.x)) as u8;
        let g = (256.0 * intensity.clamp(self.y)) as u8;
        let b = (256.0 * intensity.clamp(self.z)) as u8;

        writer.write_all(format!("{} {} {}\n", r, g, b).as_bytes())?;
        Ok(())
    }
}
