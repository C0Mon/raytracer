use std::{fs::File, io::{BufWriter, Write, Result}};

use crate::math::vector::Vec3;

pub type Colour = Vec3;

impl Colour {
    pub fn write_colour(mut writer: BufWriter<File>, pixel_colour: &Colour) -> Result<()>{
        writer.write_all(&(pixel_colour.format()).into_bytes())?;
        Ok(())
    }

    pub fn format(&self) -> String{
        format!("{0} {1} {2}\n", self.x, self.y, self.z)
    }
}
