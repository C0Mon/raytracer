use std::f64::consts::PI;
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_normalised() -> f64 {
    rand::random_range(0.0..=1.0)
}

pub fn random_float(min: f64, max: f64) -> f64 {
    rand::random_range(min..=max)
}