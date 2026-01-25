use std::{ io::{Result, Write}};

use crate::math::{interval::Interval, vector::Vec3};

pub type Colour = Vec3;

impl Colour {
    pub fn write_colour<W: Write>(&self, writer: &mut W) -> Result<()> {
        let intensity = Interval::new(0.0, 0.999);

        // Apply a linear to gamma transform for gamma 2
        let mut r = linear_to_gamma(self.x);
        let mut g = linear_to_gamma(self.y);
        let mut b = linear_to_gamma(self.z);

        // Translate the [0, 1] component to [0, 255]
        r = 256.0 * intensity.clamp(r) ;
        g = 256.0 * intensity.clamp(g);
        b = 256.0 * intensity.clamp(b);


        writer.write_all(format!("{} {} {}\n", r as u8, g as u8, b as u8).as_bytes())?;
        Ok(())
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}